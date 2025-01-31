use std::pin::Pin;

use openvr_sys2::{EVRApplicationType, EVRInitError, HmdMatrix34_t, VRChaperoneSetup, VR_Init};

fn main() {
    let mut error = EVRInitError::VRInitError_None;
    let vr_system = unsafe {
        VR_Init(&mut error, EVRApplicationType::VRApplication_Other, std::ptr::null())
    };

    if let Err(error) = error.into_result() {
        eprintln!("Failed to initialize OpenVR: {:?}", error);
        return;
    }

    let vr_system = unsafe { Pin::new_unchecked(&mut *vr_system) };
    let matrix = vr_system.GetRawZeroPoseToStandingAbsoluteTrackingPose();
    println!("{:?}", matrix.m);

    let chaperone_setup = unsafe { Pin::new_unchecked(&mut *VRChaperoneSetup()) };

    let mut matrix = unsafe { std::mem::MaybeUninit::<HmdMatrix34_t>::zeroed().assume_init() };
    unsafe {
        chaperone_setup.GetWorkingStandingZeroPoseToRawTrackingPose(&mut matrix);
    }

    println!("{:?}", matrix.m);
}
