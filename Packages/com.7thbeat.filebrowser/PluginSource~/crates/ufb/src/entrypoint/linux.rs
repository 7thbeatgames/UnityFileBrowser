use std::{
    ffi::CStr,
    ptr::{NonNull, null_mut},
};

use ufb_native::{
    IUnityGraphics, IUnityGraphicsVulkan, IUnityGraphicsVulkanV2, IUnityInterfaces,
    PFN_vkCreateWaylandSurfaceKHR, PFN_vkCreateXlibSurfaceKHR, PFN_vkGetInstanceProcAddr,
    PFN_vkVoidFunction, UnityGfxDeviceEventType,
    UnityGfxDeviceEventType_kUnityGfxDeviceEventInitialize,
    UnityGfxRenderer_kUnityGfxRendererOpenGLCore, VkAllocationCallbacks, VkInstance, VkResult,
    VkSurfaceKHR, VkWaylandSurfaceCreateInfoKHR, VkXlibSurfaceCreateInfoKHR, egl::EGL, gl::GL,
};

use crate::utils::linux::{LinuxWindowRef, set_window_handle};

static mut UNITY_INTERFACES: *mut IUnityInterfaces = null_mut();
static mut UNITY_GRAPHICS: *mut IUnityGraphics = null_mut();

static mut UNITY_GRAPHICS_VK: *mut IUnityGraphicsVulkan = null_mut();
static mut UNITY_GRAPHICS_VK2: *mut IUnityGraphicsVulkanV2 = null_mut();

fn get_interfaces<'a>() -> &'a IUnityInterfaces {
    unsafe {
        let interfaces = &*UNITY_INTERFACES;

        interfaces
    }
}

fn get_graphics<'a>() -> &'a IUnityGraphics {
    unsafe {
        let interfaces = &*UNITY_GRAPHICS;

        interfaces
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn UnityPluginLoad(interfaces: *mut IUnityInterfaces) {
    unsafe {
        UNITY_INTERFACES = interfaces;
        let interfaces = get_interfaces();
        UNITY_GRAPHICS =
            (interfaces.GetInterfaceSplit.unwrap()(0x7CBA0A9CA4DDB544, 0x8C5AD4926EB17B11))
                as *mut IUnityGraphics;

        let graphics = get_graphics();
        graphics.RegisterDeviceEventCallback.unwrap()(Some(on_graphics_device_event));
        on_graphics_device_event(UnityGfxDeviceEventType_kUnityGfxDeviceEventInitialize);
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_graphics_device_event(event_type: UnityGfxDeviceEventType) {
    if event_type == UnityGfxDeviceEventType_kUnityGfxDeviceEventInitialize {
        init_vulkan();

        let g = get_graphics();

        #[allow(non_upper_case_globals)]
        match g.GetRenderer.unwrap()() {
            UnityGfxRenderer_kUnityGfxRendererOpenGLCore => {
                init_opengl();
            }
            _ => {}
        }
    }
}

unsafe fn init_opengl() {
    unsafe {
        match GL::new("libGL.so.1") {
            Ok(gl) => {
                let display = gl.glXGetCurrentDisplay();
                if let Some(display) = NonNull::new(display as _) {
                    set_window_handle(LinuxWindowRef::Xlib {
                        window: gl.glXGetCurrentDrawable(),
                        display,
                    });

                    return;
                }
            }
            Err(e) => eprintln!("failed to load libGL: {e:?}"),
        }
        match EGL::new("libEGL.so.1") {
            Ok(egl) => {
                dbg!(egl.eglGetCurrentDisplay());
            }
            Err(e) => eprintln!("failed to load libEGL: {e:?}"),
        }
    }
}

unsafe fn init_vulkan() {
    unsafe {
        UNITY_GRAPHICS_VK =
            get_interfaces().GetInterfaceSplit.unwrap()(0x95355348d4ef4e11, 0x9789313dfcffcc87)
                as *mut IUnityGraphicsVulkan;
        UNITY_GRAPHICS_VK2 =
            get_interfaces().GetInterfaceSplit.unwrap()(0x329334C09DCA4787, 0xB347DD92A0097FFC)
                as *mut IUnityGraphicsVulkanV2;

        if UNITY_GRAPHICS_VK.is_null() || UNITY_GRAPHICS_VK2.is_null() {
            return;
        }

        let vk = &*UNITY_GRAPHICS_VK;

        vk.InterceptInitialization.unwrap()(Some(unity_vk_init), null_mut());
    }
}

static mut ORIG_GET_INSTANCE_PROC_ADDR: PFN_vkGetInstanceProcAddr = None;
static mut ORIG_CREATE_WAYLAND_SURFACE_KHR: PFN_vkCreateWaylandSurfaceKHR = None;
static mut ORIG_CREATE_XLIB_SURFACE_KHR: PFN_vkCreateXlibSurfaceKHR = None;

unsafe extern "C" fn unity_vk_init(
    orig_get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    _userdata: *mut ::std::os::raw::c_void,
) -> PFN_vkGetInstanceProcAddr {
    unsafe { ORIG_GET_INSTANCE_PROC_ADDR = orig_get_instance_proc_addr };
    Some(get_instance_proc_addr)
}

unsafe extern "C" fn get_instance_proc_addr(
    instance: VkInstance,
    p_name: *const ::std::os::raw::c_char,
) -> PFN_vkVoidFunction {
    unsafe {
        let str = CStr::from_ptr(p_name).to_str().unwrap();
        if str == "vkCreateWaylandSurfaceKHR" {
            if let Some(orig) = ORIG_GET_INSTANCE_PROC_ADDR {
                let orig_fn: PFN_vkCreateWaylandSurfaceKHR =
                    orig(instance, p_name).map(|x| std::mem::transmute(x));
                ORIG_CREATE_WAYLAND_SURFACE_KHR = orig_fn;
                println!("[UnityFileBrowser] intercepting {str}");
                return Some(std::mem::transmute(
                    vk_create_wayland_surface_khr_hook as *const (),
                ));
            }
        }
        if str == "vkCreateXlibSurfaceKHR" {
            if let Some(orig) = ORIG_GET_INSTANCE_PROC_ADDR {
                let orig_fn: PFN_vkCreateXlibSurfaceKHR =
                    orig(instance, p_name).map(|x| std::mem::transmute(x));
                ORIG_CREATE_XLIB_SURFACE_KHR = orig_fn;
                println!("[UnityFileBrowser] intercepting {str}");
                return Some(std::mem::transmute(vk_create_xlib_surface_khr as *const ()));
            }
        }

        ORIG_GET_INSTANCE_PROC_ADDR
            .map(move |x| x(instance, p_name))
            .flatten()
    }
}

unsafe extern "C" fn vk_create_wayland_surface_khr_hook(
    instance: VkInstance,
    create_info: *const VkWaylandSurfaceCreateInfoKHR,
    allocator: *const VkAllocationCallbacks,
    surface: *mut VkSurfaceKHR,
) -> VkResult {
    unsafe {
        dbg!((&*create_info).display);
        let orig_fn =
            ORIG_CREATE_WAYLAND_SURFACE_KHR.expect("vkCreateWaylandSurfaceKHR orig fn not found");

        orig_fn(instance, create_info, allocator, surface)
    }
}

unsafe extern "C" fn vk_create_xlib_surface_khr(
    instance: VkInstance,
    create_info: *const VkXlibSurfaceCreateInfoKHR,
    allocator: *const VkAllocationCallbacks,
    surface: *mut VkSurfaceKHR,
) -> VkResult {
    unsafe {
        let info = &*create_info;

        set_window_handle(LinuxWindowRef::Xlib {
            display: NonNull::new(info.dpy as _).expect("x display is null"),
            window: info.window as _,
        });

        let orig_fn =
            ORIG_CREATE_XLIB_SURFACE_KHR.expect("vkCreateWaylandSurfaceKHR orig fn not found");

        orig_fn(instance, create_info, allocator, surface)
    }
}
