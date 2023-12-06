use std::ffi::CStr;

use ash::{
    extensions::{ext::DebugUtils, khr::Surface},
    vk::{
        self, ApplicationInfo, DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT,
        DeviceCreateInfo, InstanceCreateInfo, PhysicalDevice,
    },
    Entry, Instance,
};
use exposed::{
    log::{cstr, log_warn},
    window::WindowHandle,
};
use vulkano::device::physical;

static mut INSTANCE: Option<Instance> = None;

pub struct Renderer2 {
    physical_device: PhysicalDevice,

    debug_utils: RDebug,
    instance: RInstance,
    entry: Entry,
}

impl Renderer2 {
    pub unsafe fn new(window: WindowHandle) -> Self {
        let entry = Entry::load().unwrap();

        let instance = RInstance::new(&entry);

        let debug_utils = RDebug::new(&entry, &instance.0);

        let physical_device = instance.0.enumerate_physical_devices().unwrap()[0];

        let surface = ash_window::create_surface(
            &entry,
            &instance,
            W(window).raw_display_handle(),
            W(window).raw_window_handle(),
            None,
        )
        .unwrap();

        let surface_loader = RSurface(Surface::new(&entry, &instance));

        let queue_family_index = {
            let q = instance.0.get_physical_device_queue_family_properties(physical_device);
            let mut ii = None;
            for (i, q) in q.iter().enumerate() {
                if q.queue_flags.contains(QueueFlags::GRAPHICS)
                    && surface_loader.get_physical_device_surface_support(
                        physical_device,
                        i,
                        surface,
                    )
                {
                    ii = Some(i);
                    break;
                }
            }
            ii.unwrap() as u32
        };

        let device_extension_names_raw = [Swapchain::name().as_ptr()];
        let features = vk::PhysicalDeviceFeatures { shader_clip_distance: 1, ..Default::default() };
        let priorities = [1.0];

        let queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&priorities);

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device: RDevice =
            RDevice(instance.create_device(physical_device, &device_create_info, None).unwrap());

        let present_queue: vk::Queue = device.get_device_queue(queue_family_index, 0);

        todo!()
    }

    pub unsafe fn create_surface(&mut self, window: WindowHandle) -> Result<(), ()> {
        todo!()
    }

    pub unsafe fn destroy_surface(&mut self, window: WindowHandle) -> Result<(), ()> {
        todo!()
    }

    pub unsafe fn create_swapchains(&mut self) {
        todo!()
    }

    pub unsafe fn destroy_swapchains(&mut self) {
        todo!()
    }
}

struct RInstance(pub Instance);

impl RInstance {
    pub unsafe fn new(entry: &Entry) -> Self {
        let layers = [cstr!("")];
        let extensions = [cstr!("")];

        let app_info =
            ApplicationInfo::builder().api_version(vk::make_api_version(0, 1, 0, 0)).build();

        let instance_info = InstanceCreateInfo::builder()
            .push_next(&mut RDebug::get())
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions)
            .application_info(&app_info)
            .build();

        RInstance(entry.create_instance(&instance_info, None).unwrap())
    }
}

impl Drop for RInstance {
    fn drop(&mut self) {
        unsafe { self.0.destroy_instance(None) };
    }
}

struct RDebug {
    loader: DebugUtils,
    dbg: DebugUtilsMessengerEXT,
}

impl RDebug {
    pub unsafe fn new(entry: &Entry, instance: &Instance) -> Self {
        let loader = DebugUtils::new(entry, instance);
        let dbg: DebugUtilsMessengerEXT =
            loader.create_debug_utils_messenger(&Self::get(), None).unwrap();
        Self { loader, dbg }
    }

    pub unsafe fn get() -> DebugUtilsMessengerCreateInfoEXT {
        DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(Self::vulkan_debug_callback))
            .build()
    }

    unsafe extern "system" fn vulkan_debug_callback(
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _user_data: *mut std::os::raw::c_void,
    ) -> vk::Bool32 {
        use std::borrow::Cow;
        let callback_data = *p_callback_data;
        let message_id_number = callback_data.message_id_number;

        let message_id_name = if callback_data.p_message_id_name.is_null() {
            Cow::from("")
        } else {
            CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
        };

        let message = if callback_data.p_message.is_null() {
            Cow::from("")
        } else {
            CStr::from_ptr(callback_data.p_message).to_string_lossy()
        };

        log_warn!(
           "VK_DBG_CALLBACK", "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n"
        );

        vk::FALSE
    }
}

impl Drop for RDebug {
    fn drop(&mut self) {
        unsafe { self.loader.destroy_debug_utils_messenger(self.dbg, None) }
    }
}

struct W(WindowHandle);

unsafe impl HasRawDisplayHandle for W {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        #[cfg(target_os = "android")]
        return RawDisplayHandle::Android(AndroidDisplayHandle::empty());
        #[cfg(target_os = "windows")]
        return RawDisplayHandle::Windows(WindowsDisplayHandle::empty());
    }
}

unsafe impl HasRawWindowHandle for W {
    fn raw_window_handle(&self) -> RawWindowHandle {
        #[cfg(target_os = "android")]
        return RawWindowHandle::AndroidNdk({
            let mut handle = AndroidNdkWindowHandle::empty();
            handle.a_native_window = self.0.native_handle() as _;
            handle
        });
        #[cfg(target_os = "windows")]
        return RawWindowHandle::Win32({
            let mut handle = Win32WindowHandle::empty();
            handle.hwnd = self.0.windowHandle as _;
            handle.hinstance = unsafe { exposed::window::win32::HINSTANCE } as _;
            handle
        });
    }
}

struct RSurface(Surface);