
extern crate winapi;
extern crate user32;

use winapi::*;
use user32::*;

fn main()
{
    let mut dcount = 0;
    unsafe { GetRawInputDeviceList(std::ptr::null_mut(), &mut dcount, std::mem::size_of::<RAWINPUTDEVICELIST>() as u32) };
    let mut devices = vec![unsafe { std::mem::uninitialized() }; dcount as usize];
    unsafe { GetRawInputDeviceList(devices.as_mut_ptr(), &mut dcount, std::mem::size_of::<RAWINPUTDEVICELIST>() as u32) };

    println!("RawInputDevices attached to system: ");
    for &RAWINPUTDEVICELIST { hDevice, dwType } in devices.iter()
    {
        let name =
        {
            let mut namelen = 0;
            unsafe { GetRawInputDeviceInfoW(hDevice, RIDI_DEVICENAME, std::ptr::null_mut(), &mut namelen) };
            let mut namebuf = vec![0u16; namelen as usize];
            unsafe { GetRawInputDeviceInfoW(hDevice, RIDI_DEVICENAME, std::mem::transmute(namebuf.as_mut_ptr()), &mut namelen) };
            String::from_utf16(&namebuf[..]).unwrap()
        };
        let mut devinfo: RID_DEVICE_INFO = unsafe { std::mem::uninitialized() };
        let mut devinfolen = std::mem::size_of::<RID_DEVICE_INFO>() as u32;
        unsafe { GetRawInputDeviceInfoW(hDevice, RIDI_DEVICEINFO, std::mem::transmute(&mut devinfo), &mut devinfolen) };
        match devinfo.dwType
        {
            RIM_TYPEHID => println!("Device {:?}: {} {:?}", hDevice, name, unsafe { *std::mem::transmute::<*const _, *const RID_DEVICE_INFO_HID>(&devinfo.keyboard) }),
            RIM_TYPEKEYBOARD => println!("Device {:?}: {} {:?}", hDevice, name, devinfo.keyboard),
            RIM_TYPEMOUSE => println!("Device {:?}: {} {:?}", hDevice, name, unsafe { *std::mem::transmute::<*const _, *const RID_DEVICE_INFO_MOUSE>(&devinfo.keyboard) }),
            _ => unreachable!()
        }
    }
}
