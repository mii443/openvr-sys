use std::mem::ManuallyDrop;
autocxx::include_cpp! {
    #include "../third-party/openvr/headers/openvr.h"
    generate!("vr::IVRSystem")
    generate!("vr::IVROverlay")
    generate!("vr::IVRCompositor")
    generate!("vr::IVRInput")
    generate_pod!("vr::VRActiveActionSet_t")
    generate_pod!("vr::VREvent_Reserved_t")
    generate_pod!("vr::VREvent_Controller_t")
    generate_pod!("vr::VREvent_Mouse_t")
    generate_pod!("vr::VREvent_Scroll_t")
    generate_pod!("vr::VREvent_Process_t")
    generate_pod!("vr::VREvent_Notification_t")
    generate_pod!("vr::VREvent_Overlay_t")
    generate_pod!("vr::VREvent_Status_t")
    generate_pod!("vr::VREvent_Keyboard_t")
    generate_pod!("vr::VREvent_Ipd_t")
    generate_pod!("vr::VREvent_Chaperone_t")
    generate_pod!("vr::VREvent_PerformanceTest_t")
    generate_pod!("vr::VREvent_TouchPadMove_t")
    generate_pod!("vr::VREvent_SeatedZeroPoseReset_t")
    generate_pod!("vr::VREvent_Screenshot_t")
    generate_pod!("vr::VREvent_ScreenshotProgress_t")
    generate_pod!("vr::VREvent_ApplicationLaunch_t")
    generate_pod!("vr::VREvent_EditingCameraSurface_t")
    generate_pod!("vr::VREvent_MessageOverlay_t")
    generate_pod!("vr::VREvent_Property_t")
    generate_pod!("vr::VREvent_HapticVibration_t")
    generate_pod!("vr::VREvent_WebConsole_t")
    generate_pod!("vr::VREvent_InputBindingLoad_t")
    generate_pod!("vr::VREvent_InputActionManifestLoad_t")
    generate_pod!("vr::VREvent_SpatialAnchor_t")
    generate_pod!("vr::VREvent_ProgressUpdate_t")
    generate_pod!("vr::VREvent_ShowUI_t")
    generate_pod!("vr::VREvent_ShowDevTools_t")
    generate_pod!("vr::VREvent_HDCPError_t")
    generate_pod!("vr::TrackedDeviceIndex_t")
    generate_pod!("vr::TrackedDevicePose_t")
    generate_pod!("vr::VRTextureBounds_t")
    generate_pod!("vr::VRVulkanTextureData_t")
    generate_pod!("vr::Texture_t")
    generate_pod!("vr::ETextureType")
    generate_pod!("vr::EVRInputError")
    generate_pod!("vr::HmdMatrix34_t")
    generate_pod!("vr::InputDigitalActionData_t")
    generate!("vr::IVRChaperoneSetup")
    generate!("vr::IVRChaperone")
    generate!("vr::k_ulInvalidInputValueHandle")
    generate!("vr::VR_Init")
    generate!("vr::VR_Shutdown")
    generate!("vr::VROverlay")
    generate!("vr::VRCompositor")
    generate!("vr::VRInput")
    generate!("vr::VRChaperoneSetup")
    generate!("vr::VRChaperone")
    generate!("vr::VRControllerState_t")
    generate!("vr::VRControllerState001_t")
    safety!(unsafe)
}

#[allow(non_camel_case_types, non_snake_case)]
#[repr(C)]
pub union VREvent_Data_t {
    pub reserved: ManuallyDrop<ffi::vr::VREvent_Reserved_t>,
    pub controller: ManuallyDrop<ffi::vr::VREvent_Controller_t>,
    pub mouse: ManuallyDrop<ffi::vr::VREvent_Mouse_t>,
    pub scroll: ManuallyDrop<ffi::vr::VREvent_Scroll_t>,
    pub process: ManuallyDrop<ffi::vr::VREvent_Process_t>,
    pub notification: ManuallyDrop<ffi::vr::VREvent_Notification_t>,
    pub overlay: ManuallyDrop<ffi::vr::VREvent_Overlay_t>,
    pub status: ManuallyDrop<ffi::vr::VREvent_Status_t>,
    pub keyboard: ManuallyDrop<ffi::vr::VREvent_Keyboard_t>,
    pub ipd: ManuallyDrop<ffi::vr::VREvent_Ipd_t>,
    pub chaperone: ManuallyDrop<ffi::vr::VREvent_Chaperone_t>,
    pub performanceTest: ManuallyDrop<ffi::vr::VREvent_PerformanceTest_t>,
    pub touchPadMove: ManuallyDrop<ffi::vr::VREvent_TouchPadMove_t>,
    pub seatedZeroPoseReset: ManuallyDrop<ffi::vr::VREvent_SeatedZeroPoseReset_t>,
    pub screenshot: ManuallyDrop<ffi::vr::VREvent_Screenshot_t>,
    pub screenshotProgress: ManuallyDrop<ffi::vr::VREvent_ScreenshotProgress_t>,
    pub applicationLaunch: ManuallyDrop<ffi::vr::VREvent_ApplicationLaunch_t>,
    pub cameraSurface: ManuallyDrop<ffi::vr::VREvent_EditingCameraSurface_t>,
    pub messageOverlay: ManuallyDrop<ffi::vr::VREvent_MessageOverlay_t>,
    pub property: ManuallyDrop<ffi::vr::VREvent_Property_t>,
    pub hapticVibration: ManuallyDrop<ffi::vr::VREvent_HapticVibration_t>,
    pub webConsole: ManuallyDrop<ffi::vr::VREvent_WebConsole_t>,
    pub inputBinding: ManuallyDrop<ffi::vr::VREvent_InputBindingLoad_t>,
    pub actionManifest: ManuallyDrop<ffi::vr::VREvent_InputActionManifestLoad_t>,
    pub spatialAnchor: ManuallyDrop<ffi::vr::VREvent_SpatialAnchor_t>,
    pub progressUpdate: ManuallyDrop<ffi::vr::VREvent_ProgressUpdate_t>,
    pub showUi: ManuallyDrop<ffi::vr::VREvent_ShowUI_t>,
    pub showDevTools: ManuallyDrop<ffi::vr::VREvent_ShowDevTools_t>,
    pub hdcpError: ManuallyDrop<ffi::vr::VREvent_HDCPError_t>,
}

#[allow(non_camel_case_types, non_snake_case)]
#[repr(C, packed(4))]
pub struct VREvent_t {
    pub eventType: u32,
    pub trackedDeviceIndex: ffi::vr::TrackedDeviceIndex_t,
    pub eventAgeSeconds: f32,
    pub data: VREvent_Data_t,
}

pub use ffi::vr::*;
pub use ffi::*;

impl TryFrom<u32> for EVREventType {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => EVREventType::VREvent_None,

            100 => EVREventType::VREvent_TrackedDeviceActivated,
            101 => EVREventType::VREvent_TrackedDeviceDeactivated,
            102 => EVREventType::VREvent_TrackedDeviceUpdated,
            103 => EVREventType::VREvent_TrackedDeviceUserInteractionStarted,
            104 => EVREventType::VREvent_TrackedDeviceUserInteractionEnded,
            105 => EVREventType::VREvent_IpdChanged,
            106 => EVREventType::VREvent_EnterStandbyMode,
            107 => EVREventType::VREvent_LeaveStandbyMode,
            108 => EVREventType::VREvent_TrackedDeviceRoleChanged,
            109 => EVREventType::VREvent_WatchdogWakeUpRequested,
            110 => EVREventType::VREvent_LensDistortionChanged,
            111 => EVREventType::VREvent_PropertyChanged,
            112 => EVREventType::VREvent_WirelessDisconnect,
            113 => EVREventType::VREvent_WirelessReconnect,

            200 => EVREventType::VREvent_ButtonPress, // data is controller
            201 => EVREventType::VREvent_ButtonUnpress, // data is controller
            202 => EVREventType::VREvent_ButtonTouch, // data is controller
            203 => EVREventType::VREvent_ButtonUntouch, // data is controller

            //  250 => EVREventType::VREvent_DualAnalog_Press			 // No longer sent
            //  251 => EVREventType::VREvent_DualAnalog_Unpress		 // No longer sent
            //  252 => EVREventType::VREvent_DualAnalog_Touch			 // No longer sent
            //  253 => EVREventType::VREvent_DualAnalog_Untouch		 // No longer sent
            //  254 => EVREventType::VREvent_DualAnalog_Move			 // No longer sent
            //  255 => EVREventType::VREvent_DualAnalog_ModeSwitch1	 // No longer sent
            //  256 => EVREventType::VREvent_DualAnalog_ModeSwitch2	 // No longer sent
            257 => EVREventType::VREvent_Modal_Cancel, // Sent to overlays with the

            300 => EVREventType::VREvent_MouseMove, // data is mouse
            301 => EVREventType::VREvent_MouseButtonDown, // data is mouse
            302 => EVREventType::VREvent_MouseButtonUp, // data is mouse
            303 => EVREventType::VREvent_FocusEnter, // data is overlay
            304 => EVREventType::VREvent_FocusLeave, // data is overlay
            305 => EVREventType::VREvent_ScrollDiscrete, // data is scroll
            306 => EVREventType::VREvent_TouchPadMove, // data is mouse
            307 => EVREventType::VREvent_OverlayFocusChanged, // data is overlay, global event
            308 => EVREventType::VREvent_ReloadOverlays,
            309 => EVREventType::VREvent_ScrollSmooth, // data is scroll
            310 => EVREventType::VREvent_LockMousePosition,
            311 => EVREventType::VREvent_UnlockMousePosition,

            400 => EVREventType::VREvent_InputFocusCaptured, // data is process DEPRECATED
            401 => EVREventType::VREvent_InputFocusReleased, // data is process DEPRECATED
            //  402 => EVREventType::VREvent_SceneFocusLost			, // data is process
            //  403 => EVREventType::VREvent_SceneFocusGained			, // data is process
            404 => EVREventType::VREvent_SceneApplicationChanged, // data is process - The App actually drawing the scene changed (usually to or from the compositor)
            405 => EVREventType::VREvent_SceneFocusChanged, // data is process - New app got access to draw the scene
            406 => EVREventType::VREvent_InputFocusChanged, // data is process
            //  407 => EVREventType::VREvent_SceneApplicationSecondaryRenderingStarted ,
            408 => EVREventType::VREvent_SceneApplicationUsingWrongGraphicsAdapter, // data is process
            409 => EVREventType::VREvent_ActionBindingReloaded, // data is process - The App that action binds reloaded for

            410 => EVREventType::VREvent_HideRenderModels, // Sent to the scene application to request hiding render models temporarily
            411 => EVREventType::VREvent_ShowRenderModels, // Sent to the scene application to request restoring render model visibility

            412 => EVREventType::VREvent_SceneApplicationStateChanged, // No data; but query VRApplications()->GetSceneApplicationState();

            420 => EVREventType::VREvent_ConsoleOpened,
            421 => EVREventType::VREvent_ConsoleClosed,

            500 => EVREventType::VREvent_OverlayShown,
            501 => EVREventType::VREvent_OverlayHidden,
            502 => EVREventType::VREvent_DashboardActivated,
            503 => EVREventType::VREvent_DashboardDeactivated,
            // 504 => EVREventType::VREvent_DashboardThumbSelected		, // Sent to the overlay manager - data is overlay - No longer sent
            505 => EVREventType::VREvent_DashboardRequested, // Sent to the overlay manager - data is overlay
            506 => EVREventType::VREvent_ResetDashboard,     // Send to the overlay manager
            // 507 => EVREventType::VREvent_RenderToast					, // Send to the dashboard to render a toast - data is the notification ID -- no longer sent
            508 => EVREventType::VREvent_ImageLoaded, // Sent to overlays when a SetOverlayRaw or SetOverlayFromFile call finishes loading
            509 => EVREventType::VREvent_ShowKeyboard, // Sent to keyboard renderer in the dashboard to invoke it
            510 => EVREventType::VREvent_HideKeyboard, // Sent to keyboard renderer in the dashboard to hide it
            511 => EVREventType::VREvent_OverlayGamepadFocusGained, // Sent to an overlay when IVROverlay::SetFocusOverlay is called on it
            512 => EVREventType::VREvent_OverlayGamepadFocusLost, // Send to an overlay when it previously had focus and IVROverlay::SetFocusOverlay is called on something else
            513 => EVREventType::VREvent_OverlaySharedTextureChanged,
            // 514 => EVREventType::VREvent_DashboardGuideButtonDown	, // These are no longer sent
            // 515 => EVREventType::VREvent_DashboardGuideButtonUp		,
            516 => EVREventType::VREvent_ScreenshotTriggered, // Screenshot button combo was pressed, Dashboard should request a screenshot
            517 => EVREventType::VREvent_ImageFailed, // Sent to overlays when a SetOverlayRaw or SetOverlayfromFail fails to load
            518 => EVREventType::VREvent_DashboardOverlayCreated,
            519 => EVREventType::VREvent_SwitchGamepadFocus,

            // Screenshot API
            520 => EVREventType::VREvent_RequestScreenshot, // Sent by vrclient application to compositor to take a screenshot
            521 => EVREventType::VREvent_ScreenshotTaken, // Sent by compositor to the application that the screenshot has been taken
            522 => EVREventType::VREvent_ScreenshotFailed, // Sent by compositor to the application that the screenshot failed to be taken
            523 => EVREventType::VREvent_SubmitScreenshotToDashboard, // Sent by compositor to the dashboard that a completed screenshot was submitted
            524 => EVREventType::VREvent_ScreenshotProgressToDashboard, // Sent by compositor to the dashboard that a completed screenshot was submitted

            525 => EVREventType::VREvent_PrimaryDashboardDeviceChanged,
            526 => EVREventType::VREvent_RoomViewShown, // Sent by compositor whenever room-view is enabled
            527 => EVREventType::VREvent_RoomViewHidden, // Sent by compositor whenever room-view is disabled
            528 => EVREventType::VREvent_ShowUI,         // data is showUi
            529 => EVREventType::VREvent_ShowDevTools,   // data is showDevTools
            530 => EVREventType::VREvent_DesktopViewUpdating,
            531 => EVREventType::VREvent_DesktopViewReady,

            532 => EVREventType::VREvent_StartDashboard,
            533 => EVREventType::VREvent_ElevatePrism,

            534 => EVREventType::VREvent_OverlayClosed,

            600 => EVREventType::VREvent_Notification_Shown,
            601 => EVREventType::VREvent_Notification_Hidden,
            602 => EVREventType::VREvent_Notification_BeginInteraction,
            603 => EVREventType::VREvent_Notification_Destroyed,

            700 => EVREventType::VREvent_Quit, // data is process
            701 => EVREventType::VREvent_ProcessQuit, // data is process
            // 702 => EVREventType::VREvent_QuitAborted_UserPrompt			, // data is process
            703 => EVREventType::VREvent_QuitAcknowledged, // data is process
            704 => EVREventType::VREvent_DriverRequestedQuit, // The driver has requested that SteamVR shut down
            705 => EVREventType::VREvent_RestartRequested, // A driver or other component wants the user to restart SteamVR
            706 => EVREventType::VREvent_InvalidateSwapTextureSets,

            800 => EVREventType::VREvent_ChaperoneDataHasChanged, // this will never happen with the new chaperone system
            801 => EVREventType::VREvent_ChaperoneUniverseHasChanged,
            802 => EVREventType::VREvent_ChaperoneTempDataHasChanged, // this will never happen with the new chaperone system
            803 => EVREventType::VREvent_ChaperoneSettingsHaveChanged,
            804 => EVREventType::VREvent_SeatedZeroPoseReset,
            805 => EVREventType::VREvent_ChaperoneFlushCache, // Sent when the process needs to reload any cached data it retrieved from VRChaperone()
            806 => EVREventType::VREvent_ChaperoneRoomSetupStarting, // Triggered by CVRChaperoneClient::RoomSetupStarting
            807 => EVREventType::VREvent_ChaperoneRoomSetupFinished, // Triggered by CVRChaperoneClient::CommitWorkingCopy
            808 => EVREventType::VREvent_StandingZeroPoseReset,

            820 => EVREventType::VREvent_AudioSettingsHaveChanged,

            850 => EVREventType::VREvent_BackgroundSettingHasChanged,
            851 => EVREventType::VREvent_CameraSettingsHaveChanged,
            852 => EVREventType::VREvent_ReprojectionSettingHasChanged,
            853 => EVREventType::VREvent_ModelSkinSettingsHaveChanged,
            854 => EVREventType::VREvent_EnvironmentSettingsHaveChanged,
            855 => EVREventType::VREvent_PowerSettingsHaveChanged,
            856 => EVREventType::VREvent_EnableHomeAppSettingsHaveChanged,
            857 => EVREventType::VREvent_SteamVRSectionSettingChanged,
            858 => EVREventType::VREvent_LighthouseSectionSettingChanged,
            859 => EVREventType::VREvent_NullSectionSettingChanged,
            860 => EVREventType::VREvent_UserInterfaceSectionSettingChanged,
            861 => EVREventType::VREvent_NotificationsSectionSettingChanged,
            862 => EVREventType::VREvent_KeyboardSectionSettingChanged,
            863 => EVREventType::VREvent_PerfSectionSettingChanged,
            864 => EVREventType::VREvent_DashboardSectionSettingChanged,
            865 => EVREventType::VREvent_WebInterfaceSectionSettingChanged,
            866 => EVREventType::VREvent_TrackersSectionSettingChanged,
            867 => EVREventType::VREvent_LastKnownSectionSettingChanged,
            868 => EVREventType::VREvent_DismissedWarningsSectionSettingChanged,
            869 => EVREventType::VREvent_GpuSpeedSectionSettingChanged,
            870 => EVREventType::VREvent_WindowsMRSectionSettingChanged,
            871 => EVREventType::VREvent_OtherSectionSettingChanged,

            900 => EVREventType::VREvent_StatusUpdate,

            950 => EVREventType::VREvent_WebInterface_InstallDriverCompleted,

            1000 => EVREventType::VREvent_MCImageUpdated,

            1100 => EVREventType::VREvent_FirmwareUpdateStarted,
            1101 => EVREventType::VREvent_FirmwareUpdateFinished,

            1200 => EVREventType::VREvent_KeyboardClosed,
            1201 => EVREventType::VREvent_KeyboardCharInput,
            1202 => EVREventType::VREvent_KeyboardDone, // Sent when DONE button clicked on keyboard

            // 1300 => EVREventType::VREvent_ApplicationTransitionStarted		,
            // 1301 => EVREventType::VREvent_ApplicationTransitionAborted		,
            // 1302 => EVREventType::VREvent_ApplicationTransitionNewAppStarted	,
            1303 => EVREventType::VREvent_ApplicationListUpdated,
            1304 => EVREventType::VREvent_ApplicationMimeTypeLoad,
            //  1305 => EVREventType::VREvent_ApplicationTransitionNewAppLaunchComplete ,
            1306 => EVREventType::VREvent_ProcessConnected,
            1307 => EVREventType::VREvent_ProcessDisconnected,

            // 1400 => EVREventType::VREvent_Compositor_MirrorWindowShown		, // DEPRECATED
            // 1401 => EVREventType::VREvent_Compositor_MirrorWindowHidden		, // DEPRECATED
            1410 => EVREventType::VREvent_Compositor_ChaperoneBoundsShown,
            1411 => EVREventType::VREvent_Compositor_ChaperoneBoundsHidden,
            1412 => EVREventType::VREvent_Compositor_DisplayDisconnected,
            1413 => EVREventType::VREvent_Compositor_DisplayReconnected,
            1414 => EVREventType::VREvent_Compositor_HDCPError, // data is hdcpError
            1415 => EVREventType::VREvent_Compositor_ApplicationNotResponding,
            1416 => EVREventType::VREvent_Compositor_ApplicationResumed,
            1417 => EVREventType::VREvent_Compositor_OutOfVideoMemory,
            1418 => EVREventType::VREvent_Compositor_DisplayModeNotSupported, // k_pch_SteamVR_PreferredRefreshRate
            1419 => EVREventType::VREvent_Compositor_StageOverrideReady,
            1420 => EVREventType::VREvent_Compositor_RequestDisconnectReconnect,

            1500 => EVREventType::VREvent_TrackedCamera_StartVideoStream,
            1501 => EVREventType::VREvent_TrackedCamera_StopVideoStream,
            1502 => EVREventType::VREvent_TrackedCamera_PauseVideoStream,
            1503 => EVREventType::VREvent_TrackedCamera_ResumeVideoStream,
            1550 => EVREventType::VREvent_TrackedCamera_EditingSurface,

            1600 => EVREventType::VREvent_PerformanceTest_EnableCapture,
            1601 => EVREventType::VREvent_PerformanceTest_DisableCapture,
            1602 => EVREventType::VREvent_PerformanceTest_FidelityLevel,

            1650 => EVREventType::VREvent_MessageOverlay_Closed,
            1651 => EVREventType::VREvent_MessageOverlayCloseRequested,

            1700 => EVREventType::VREvent_Input_HapticVibration, // data is hapticVibration
            1701 => EVREventType::VREvent_Input_BindingLoadFailed, // data is inputBinding
            1702 => EVREventType::VREvent_Input_BindingLoadSuccessful, // data is inputBinding
            1703 => EVREventType::VREvent_Input_ActionManifestReloaded, // no data
            1704 => EVREventType::VREvent_Input_ActionManifestLoadFailed, // data is actionManifest
            1705 => EVREventType::VREvent_Input_ProgressUpdate,  // data is progressUpdate
            1706 => EVREventType::VREvent_Input_TrackerActivated,
            1707 => EVREventType::VREvent_Input_BindingsUpdated,
            1708 => EVREventType::VREvent_Input_BindingSubscriptionChanged,

            1800 => EVREventType::VREvent_SpatialAnchors_PoseUpdated, // data is spatialAnchor. broadcast
            1801 => EVREventType::VREvent_SpatialAnchors_DescriptorUpdated, // data is spatialAnchor. broadcast
            1802 => EVREventType::VREvent_SpatialAnchors_RequestPoseUpdate, // data is spatialAnchor. sent to specific driver
            1803 => EVREventType::VREvent_SpatialAnchors_RequestDescriptorUpdate, // data is spatialAnchor. sent to specific driver

            1900 => EVREventType::VREvent_SystemReport_Started, // user or system initiated generation of a system report. broadcast

            2000 => EVREventType::VREvent_Monitor_ShowHeadsetView, // data is process
            2001 => EVREventType::VREvent_Monitor_HideHeadsetView, // data is process

            // Vendors are free to expose private events in this reserved region
            10000 => EVREventType::VREvent_VendorSpecific_Reserved_Start,
            19999 => EVREventType::VREvent_VendorSpecific_Reserved_End,
            _ => return Err(()),
        })
    }
}

impl EVROverlayError {
    pub fn into_result(self) -> Result<(), Self> {
        if self == EVROverlayError::VROverlayError_None {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl std::fmt::Debug for EVROverlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VROverlayError_None => write!(f, "VROverlayError_None"),
            Self::VROverlayError_UnknownOverlay => write!(f, "VROverlayError_UnknownOverlay"),
            Self::VROverlayError_InvalidHandle => write!(f, "VROverlayError_InvalidHandle"),
            Self::VROverlayError_PermissionDenied => write!(f, "VROverlayError_PermissionDenied"),
            Self::VROverlayError_OverlayLimitExceeded => {
                write!(f, "VROverlayError_OverlayLimitExceeded")
            }
            Self::VROverlayError_WrongVisibilityType => {
                write!(f, "VROverlayError_WrongVisibilityType")
            }
            Self::VROverlayError_KeyTooLong => write!(f, "VROverlayError_KeyTooLong"),
            Self::VROverlayError_NameTooLong => write!(f, "VROverlayError_NameTooLong"),
            Self::VROverlayError_KeyInUse => write!(f, "VROverlayError_KeyInUse"),
            Self::VROverlayError_WrongTransformType => {
                write!(f, "VROverlayError_WrongTransformType")
            }
            Self::VROverlayError_InvalidTrackedDevice => {
                write!(f, "VROverlayError_InvalidTrackedDevice")
            }
            Self::VROverlayError_InvalidParameter => write!(f, "VROverlayError_InvalidParameter"),
            Self::VROverlayError_ThumbnailCantBeDestroyed => {
                write!(f, "VROverlayError_ThumbnailCantBeDestroyed")
            }
            Self::VROverlayError_ArrayTooSmall => write!(f, "VROverlayError_ArrayTooSmall"),
            Self::VROverlayError_RequestFailed => write!(f, "VROverlayError_RequestFailed"),
            Self::VROverlayError_InvalidTexture => write!(f, "VROverlayError_InvalidTexture"),
            Self::VROverlayError_UnableToLoadFile => write!(f, "VROverlayError_UnableToLoadFile"),
            Self::VROverlayError_KeyboardAlreadyInUse => {
                write!(f, "VROverlayError_KeyboardAlreadyInUse")
            }
            Self::VROverlayError_NoNeighbor => write!(f, "VROverlayError_NoNeighbor"),
            Self::VROverlayError_TooManyMaskPrimitives => {
                write!(f, "VROverlayError_TooManyMaskPrimitives")
            }
            Self::VROverlayError_BadMaskPrimitive => write!(f, "VROverlayError_BadMaskPrimitive"),
            Self::VROverlayError_TextureAlreadyLocked => {
                write!(f, "VROverlayError_TextureAlreadyLocked")
            }
            Self::VROverlayError_TextureLockCapacityReached => {
                write!(f, "VROverlayError_TextureLockCapacityReached")
            }
            Self::VROverlayError_TextureNotLocked => write!(f, "VROverlayError_TextureNotLocked"),
            Self::VROverlayError_TimedOut => write!(f, "VROverlayError_TimedOut"),
        }
    }
}

impl std::fmt::Display for EVROverlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for EVROverlayError {}

impl std::fmt::Debug for ETrackedPropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ETrackedPropertyError::*;
        match self {
            TrackedProp_Success => write!(f, "TrackedProp_Success"),
            TrackedProp_WrongDataType => write!(f, "TrackedProp_WrongDataType"),
            TrackedProp_WrongDeviceClass => write!(f, "TrackedProp_WrongDeviceClass"),
            TrackedProp_BufferTooSmall => write!(f, "TrackedProp_BufferTooSmall"),
            TrackedProp_UnknownProperty => write!(f, "TrackedProp_UnknownProperty"),
            TrackedProp_InvalidDevice => write!(f, "TrackedProp_InvalidDevice"),
            TrackedProp_CouldNotContactServer => write!(f, "TrackedProp_CouldNotContactServer"),
            TrackedProp_ValueNotProvidedByDevice => {
                write!(f, "TrackedProp_ValueNotProvidedByDevice")
            }
            TrackedProp_StringExceedsMaximumLength => {
                write!(f, "TrackedProp_StringExceedsMaximumLength")
            }
            TrackedProp_NotYetAvailable => write!(f, "TrackedProp_NotYetAvailable"),
            TrackedProp_PermissionDenied => write!(f, "TrackedProp_PermissionDenied"),
            TrackedProp_InvalidOperation => write!(f, "TrackedProp_InvalidOperation"),
            TrackedProp_CannotWriteToWildcards => write!(f, "TrackedProp_CannotWriteToWildcards"),
            TrackedProp_IPCReadFailure => write!(f, "TrackedProp_IPCReadFailure"),
            TrackedProp_OutOfMemory => write!(f, "TrackedProp_OutOfMemory"),
            TrackedProp_InvalidContainer => write!(f, "TrackedProp_InvalidContainer"),
        }
    }
}

impl std::fmt::Display for ETrackedPropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl ETrackedPropertyError {
    pub fn into_result(self) -> Result<(), Self> {
        if self == ETrackedPropertyError::TrackedProp_Success {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl std::error::Error for ETrackedPropertyError {}

impl std::fmt::Debug for EVRInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EVRInputError::*;
        match self {
            VRInputError_None => write!(f, "VRInputError_None"),
            VRInputError_NameNotFound => write!(f, "VRInputError_NameNotFound"),
            VRInputError_WrongType => write!(f, "VRInputError_WrongType"),
            VRInputError_InvalidHandle => write!(f, "VRInputError_InvalidHandle"),
            VRInputError_InvalidParam => write!(f, "VRInputError_InvalidParam"),
            VRInputError_NoSteam => write!(f, "VRInputError_NoSteam"),
            VRInputError_MaxCapacityReached => write!(f, "VRInputError_MaxCapacityReached"),
            VRInputError_IPCError => write!(f, "VRInputError_IPCError"),
            VRInputError_NoActiveActionSet => write!(f, "VRInputError_NoActiveActionSet"),
            VRInputError_InvalidDevice => write!(f, "VRInputError_InvalidDevice"),
            VRInputError_InvalidSkeleton => write!(f, "VRInputError_InvalidSkeleton"),
            VRInputError_InvalidBoneCount => write!(f, "VRInputError_InvalidBoneCount"),
            VRInputError_InvalidCompressedData => write!(f, "VRInputError_InvalidCompressedData"),
            VRInputError_NoData => write!(f, "VRInputError_NoData"),
            VRInputError_BufferTooSmall => write!(f, "VRInputError_BufferTooSmall"),
            VRInputError_MismatchedActionManifest => {
                write!(f, "VRInputError_MismatchedActionManifest")
            }
            VRInputError_MissingSkeletonData => write!(f, "VRInputError_MissingSkeletonData"),
            VRInputError_InvalidBoneIndex => write!(f, "VRInputError_InvalidBoneIndex"),
            VRInputError_InvalidPriority => write!(f, "VRInputError_InvalidPriority"),
            VRInputError_PermissionDenied => write!(f, "VRInputError_PermissionDenied"),
            VRInputError_InvalidRenderModel => write!(f, "VRInputError_InvalidRenderModel"),
        }
    }
}

impl std::fmt::Display for EVRInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for EVRInputError {}

impl EVRInputError {
    pub fn into_result(self) -> Result<(), Self> {
        if self == EVRInputError::VRInputError_None {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl EVRInitError {
    pub fn into_result(self) -> Result<(), Self> {
        if self == EVRInitError::VRInitError_None {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl std::fmt::Debug for EVRInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "VRInitError_Unknown"),
        }
    }
}

impl std::fmt::Display for EVRInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for EVRInitError {}

impl<T: num::NumCast + num::Float + num::Zero + num::One + nalgebra::Scalar> From<&'_ HmdMatrix34_t>
    for nalgebra::Matrix4<T>
{
    fn from(m: &HmdMatrix34_t) -> nalgebra::Matrix4<T> {
        // Note: [[float; 4]; 4] -> Matrix is column major
        let mut tmp = [[T::nan(); 4]; 4];
        for i in 0..3 {
            for j in 0..4 {
                tmp[j][i] = T::from(m.m[i][j]).unwrap();
            }
        }
        for i in 0..3 {
            tmp[i][3] = T::zero();
        }
        tmp[3][3] = T::one();
        tmp.into()
    }
}

impl<T: num::NumCast + num::Float + num::Zero + num::One + nalgebra::Scalar> From<HmdMatrix34_t>
    for nalgebra::Matrix4<T>
{
    fn from(m: HmdMatrix34_t) -> nalgebra::Matrix4<T> {
        Self::from(&m)
    }
}

impl<T: num::ToPrimitive> From<&'_ nalgebra::Matrix4<T>> for HmdMatrix34_t {
    fn from(m: &nalgebra::Matrix4<T>) -> Self {
        let mut ret = unsafe { std::mem::MaybeUninit::<Self>::zeroed().assume_init() };
        for i in 0..3 {
            for j in 0..4 {
                ret.m[i][j] = m[(i, j)].to_f32().unwrap();
            }
        }
        ret
    }
}
