use std::collections::HashMap;
use std::num::NonZeroU32;

use windows::Win32::Foundation::{CloseHandle, GetLastError};
use windows::Win32::Media::Audio::ISimpleAudioVolume;

use windows::core::ComInterface;
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_DeviceDesc;
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

use windows::Win32::Media::Audio::{
    self, eMultimedia, eRender, IAudioSessionControl2, IAudioSessionManager2, IMMDevice,
    IMMDeviceEnumerator, DEVICE_STATE_ACTIVE,
};
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL, STGM_READ};

pub type ProcessId = NonZeroU32;
pub type WindowsResult<T> = windows::core::Result<T>;

pub struct ApplicationAudio {
    process_id: ProcessId,
    session: ISimpleAudioVolume,
}
impl ApplicationAudio {
    pub fn new(process_id: ProcessId, session: ISimpleAudioVolume) -> Self {
        Self {
            process_id,
            session,
        }
    }

    pub fn process_id(&self) -> ProcessId {
        self.process_id
    }

    pub fn get_process_path(&self) -> WindowsResult<String> {
        unsafe {
            let handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                false,
                self.process_id.into(),
            )?;
            // The rust wrapper around `GetModuleFileNameExW` ensures that the string doesn't overflow, and is null-terminated.
            let mut filename = [0u16; 1024];
            let length = GetModuleFileNameExW(handle, None, &mut filename);
            CloseHandle(handle)?;
            if length == 0 {
                match GetLastError() {
                    Err(e) => return Err(e),
                    Ok(_) => panic!("No error returned by GetLastError"),
                }
            }
            Ok(String::from_utf16(&filename)?)
        }
    }

    pub fn get_volume(&self) -> WindowsResult<f32> {
        unsafe { self.session.GetMasterVolume() }
    }
    pub fn set_volume(&mut self, volume: f32) -> WindowsResult<()> {
        unsafe { self.session.SetMasterVolume(volume, std::ptr::null()) }
    }

    pub fn set_mute(&mut self, mute: bool) -> WindowsResult<()> {
        unsafe { self.session.SetMute(mute, std::ptr::null()) }
    }
}

unsafe fn get_audio_device_enumerator() -> WindowsResult<IMMDeviceEnumerator> {
    CoInitialize(None)?;
    CoCreateInstance(&Audio::MMDeviceEnumerator, None, CLSCTX_ALL)
}

pub fn get_audio_devices() -> WindowsResult<HashMap<String, IMMDevice>> {
    unsafe {
        let enumerator = get_audio_device_enumerator()?;
        // Get all active output devices.
        let devices = enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;

        let mut result_devices = HashMap::new();
        for i in 0..devices.GetCount()? {
            let device = devices.Item(i)?;
            let properties = device.OpenPropertyStore(STGM_READ)?;
            let device_desc_container = properties.GetValue(&PKEY_Device_DeviceDesc)?;
            let device_desc = device_desc_container.Anonymous.Anonymous.Anonymous.pwszVal;

            result_devices.insert(device_desc.to_string()?, device);
        }
        Ok(result_devices)
    }
}

pub fn get_audio_sessions() -> WindowsResult<Vec<ApplicationAudio>> {
    unsafe {
        let device_enumerator = get_audio_device_enumerator()?;
        let output_device = device_enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;

        let session_manager: IAudioSessionManager2 = output_device.Activate(CLSCTX_ALL, None)?;
        let session_enumerator = session_manager.GetSessionEnumerator()?;

        let mut result_sessions = vec![];
        for i in 0..session_enumerator.GetCount()? {
            let session_control = session_enumerator.GetSession(i)?;
            let session: IAudioSessionControl2 = session_control.cast()?;
            let Some(process_id) = NonZeroU32::new(session.GetProcessId()?) else {
                // System service or otherwise inaccessible from current user, don't want to process it.
                continue;
            };
            result_sessions.push(ApplicationAudio::new(process_id, session.cast()?));
        }

        Ok(result_sessions)
    }
}
