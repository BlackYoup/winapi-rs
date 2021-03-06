// Copyright © 2015; Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi.h
use ctypes::{c_void};
use shared::basetsd::{UINT64, SIZE_T};
use shared::dxgiformat::{DXGI_FORMAT};
use shared::dxgitype::{DXGI_USAGE, DXGI_MODE_DESC, DXGI_GAMMA_CONTROL_CAPABILITIES, DXGI_GAMMA_CONTROL, DXGI_MODE_ROTATION, DXGI_SAMPLE_DESC};
use shared::guiddef::{REFIID, REFGUID};
use shared::minwindef::{UINT, BOOL, FLOAT, HMODULE, DWORD, BYTE};
use shared::windef::{HWND, HDC, RECT, HMONITOR};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LUID, WCHAR, INT, LARGE_INTEGER, HANDLE};

STRUCT!{struct DXGI_FRAME_STATISTICS {
    PresentCount: UINT,
    PresentRefreshCount: UINT,
    SyncRefreshCount: UINT,
    SyncQPCTime: LARGE_INTEGER,
    SyncGPUTime: LARGE_INTEGER,
}}
STRUCT!{struct DXGI_MAPPED_RECT {
    Pitch: INT,
    pBits: *mut BYTE,
}}
STRUCT!{struct DXGI_ADAPTER_DESC {
    Description: [WCHAR; 128],
    VectorId: UINT,
    DeviceId: UINT,
    SubSysId: UINT,
    Revision: UINT,
    DedicatedVideoMemory: SIZE_T,
    DedicatedSystemMemory: SIZE_T,
    SharedSystemMemory: SIZE_T,
    AdapterLuid: LUID,
}}
STRUCT!{struct DXGI_OUTPUT_DESC {
    DeviceName: [WCHAR; 32],
    DesktopCoordinates: RECT,
    AttachedToDesktop: BOOL,
    Rotation: DXGI_MODE_ROTATION,
    Monitor: HMONITOR,
}}
STRUCT!{struct DXGI_SHARED_RESOURCE {
    Handle: HANDLE,
}}
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: DWORD = 0x28000000;
pub const DXGI_RESOURCE_PRIORITY_LOW: DWORD = 0x50000000;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: DWORD = 0x78000000;
pub const DXGI_RESOURCE_PRIORITY_HIGH: DWORD = 0xa0000000;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: DWORD = 0xc8000000;
ENUM!{enum DXGI_RESIDENCY {
    DXGI_RESIDENCY_FULLY_RESIDENT = 1,
    DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY = 2,
    DXGI_RESIDENCY_EVICTED_TO_DISK = 3,
}}
STRUCT!{struct DXGI_SURFACE_DESC {
    Width: UINT,
    Height: UINT,
    Format: DXGI_FORMAT,
    SampleDesc: DXGI_SAMPLE_DESC,
}}
ENUM!{enum DXGI_SWAP_EFFECT {
    DXGI_SWAP_EFFECT_DISCARD = 0,
    DXGI_SWAP_EFFECT_SEQUENTIAL = 1,
    DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL = 3,
    DXGI_SWAP_EFFECT_FLIP_DISCARD = 4,
}}
ENUM!{enum DXGI_SWAP_CHAIN_FLAG {
    DXGI_SWAP_CHAIN_FLAG_NONPREROTATED = 1,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH = 2,
    DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE = 4,
    DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT = 8,
    DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER = 16,
    DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY = 32,
    DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT = 64,
    DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER = 128,
    DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO = 256,
    DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO = 512,
}}
STRUCT!{struct DXGI_SWAP_CHAIN_DESC {
    BufferDesc: DXGI_MODE_DESC,
    SampleDesc: DXGI_SAMPLE_DESC,
    BufferUsage: DXGI_USAGE,
    BufferCount: UINT,
    OutputWindow: HWND,
    Windowed: BOOL,
    SwapEffect: DXGI_SWAP_EFFECT,
    Flags: UINT,
}}
RIDL!(
interface IDXGIObject(IDXGIObjectVtbl): IUnknown(IUnknownVtbl) {
    fn SetPrivateData(
        &self, Name: REFGUID, DataSize: UINT, pData: *const c_void
    ) -> HRESULT,
    fn SetPrivateDataInterface(&self, Name: REFGUID, pUnknown: *const IUnknown) -> HRESULT,
    fn GetPrivateData(
        &self, Name: REFGUID, pDataSize: *mut UINT, pData: *mut c_void
    ) -> HRESULT,
    fn GetParent(
        &self, riid: REFIID, ppParent: *mut *mut c_void
    ) -> HRESULT
});
RIDL!(
interface IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDevice(&self, riid: REFIID, ppDevice: *mut *mut c_void) -> HRESULT
});
RIDL!(
interface IDXGIResource(IDXGIResourceVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn GetSharedHandle(&self, pSharedHandle: *mut HANDLE) -> HRESULT,
    fn GetUsage(&self, pUsage: *mut DXGI_USAGE) -> HRESULT,
    fn SetEvictionPriority(&self, EvictionPriority: UINT) -> HRESULT,
    fn GetEvictionPriority(&self, pEvictionPriority: *mut UINT) -> HRESULT
});
RIDL!(
interface IDXGIKeyedMutex(IDXGIKeyedMutexVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn AcquireSync(&self, Key: UINT64, dwMilliseconds: DWORD) -> HRESULT,
    fn ReleaseSync(&self, Key: UINT64) -> HRESULT
});
RIDL!(
interface IDXGISurface(IDXGISurfaceVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn GetDesc(&self, pDesc: *mut DXGI_SURFACE_DESC) -> HRESULT,
    fn Map(&self, pLockedRect: *mut DXGI_MAPPED_RECT, MapFlags: UINT) -> HRESULT,
    fn Unmap(&self) -> HRESULT
});
RIDL!(
interface IDXGISurface1(IDXGISurface1Vtbl): IDXGISurface(IDXGISurfaceVtbl) {
    fn GetDC(&self, Discard: BOOL, phdc: *mut HDC) -> HRESULT,
    fn ReleaseDC(&self, pDirtyRect: *mut RECT) -> HRESULT
});
RIDL!(
interface IDXGIAdapter(IDXGIAdapterVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn EnumOutputs(&self, Output: UINT, ppOutput: *mut *mut IDXGIOutput) -> HRESULT,
    fn GetDesc(&self, pDesc: *mut DXGI_ADAPTER_DESC) -> HRESULT,
    fn CheckInterfaceSupport(
        &self, InterfaceName: REFGUID, pUMDVersion: *mut LARGE_INTEGER
    ) -> HRESULT
});
RIDL!(
interface IDXGIOutput(IDXGIOutputVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDesc(&self, pDesc: *mut DXGI_OUTPUT_DESC) -> HRESULT,
    fn GetDisplayModeList(
        &self, EnumFormat: DXGI_FORMAT, Flags: UINT, pNumModes: *mut UINT,
        pDesc: *mut DXGI_MODE_DESC
    ) -> HRESULT,
    fn FindClosestMatchingMode(
        &self, pModeToMatch: *const DXGI_MODE_DESC, pClosestMatch: *mut DXGI_MODE_DESC,
        pConcernedDevice: *mut IUnknown
    ) -> HRESULT,
    fn WaitForVBlank(&self) -> HRESULT,
    fn TakeOwnership(&self, pDevice: *mut IUnknown, Exclusive: BOOL) -> HRESULT,
    fn ReleaseOwnership(&self) -> (),
    fn GetGammaControlCapabilities(
        &self, pGammaCaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES
    ) -> HRESULT,
    fn SetGammaControl(&self, pArray: *const DXGI_GAMMA_CONTROL) -> HRESULT,
    fn GetGammaControl(&self, pArray: *mut DXGI_GAMMA_CONTROL) -> HRESULT,
    fn SetDisplaySurface(&self, pScanoutSurface: *mut IDXGISurface) -> HRESULT,
    fn GetDisplaySurfaceData(&self, pDestination: *mut IDXGISurface) -> HRESULT,
    fn GetFrameStatistics(&self, pStats: *mut DXGI_FRAME_STATISTICS) -> HRESULT
});
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: DWORD = 16;
pub const DXGI_PRESENT_TEST: DWORD = 0x00000001;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: DWORD = 0x00000002;
pub const DXGI_PRESENT_RESTART: DWORD = 0x00000004;
pub const DXGI_PRESENT_DO_NOT_WAIT: DWORD = 0x00000008;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: DWORD = 0x00000010;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: DWORD = 0x00000020;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: DWORD = 0x00000040;
pub const DXGI_PRESENT_USE_DURATION: DWORD = 0x00000100;
RIDL!(
interface IDXGISwapChain(IDXGISwapChainVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn Present(&self, SyncInterval: UINT, Flags: UINT) -> HRESULT,
    fn GetBuffer(
        &self, Buffer: UINT, riid: REFIID, ppSurface: *mut *mut c_void
    ) -> HRESULT,
    fn SetFullscreenState(&self, Fullscreen: BOOL, pTarget: *mut IDXGIOutput) -> HRESULT,
    fn GetFullscreenState(
        &self, pFullscreen: *mut BOOL, ppTarget: *mut *mut IDXGIOutput
    ) -> HRESULT,
    fn GetDesc(&self, pDesc: *mut DXGI_SWAP_CHAIN_DESC) -> HRESULT,
    fn ResizeBuffers(
        &self, BufferCount: UINT, Width: UINT, Height: UINT, NewFormat: DXGI_FORMAT,
        SwapChainFlags: UINT
    ) -> HRESULT,
    fn ResizeTarget(&self, pNewTargetParameters: *const DXGI_MODE_DESC) -> HRESULT,
    fn GetContainingOutput(&self, ppOutput: *mut *mut IDXGIOutput) -> HRESULT,
    fn GetFrameStatistics(&self, pStats: *mut DXGI_FRAME_STATISTICS) -> HRESULT,
    fn GetLastPresentCount(&self, pLastPresentCount: *mut UINT) -> HRESULT
});
RIDL!(
interface IDXGIFactory(IDXGIFactoryVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn EnumAdapters(&self, Adapter: UINT, ppAdapter: *mut *mut IDXGIAdapter) -> HRESULT,
    fn MakeWindowAssociation(&self, WindowHandle: HWND, Flags: UINT) -> HRESULT,
    fn GetWindowAssociation(&self, pWindowHandle: *mut HWND) -> HRESULT,
    fn CreateSwapChain(
        &self, pDevice: *mut IUnknown, pDesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppSwapChain: *mut *mut IDXGISwapChain
    ) -> HRESULT,
    fn CreateSoftwareAdapter(
        &self, Module: HMODULE, ppAdapter: *mut *mut IDXGIAdapter
    ) -> HRESULT
});
RIDL!(
interface IDXGIDevice(IDXGIDeviceVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetAdapter(&self, pAdapter: *mut *mut IDXGIAdapter) -> HRESULT,
    fn CreateSurface(
        &self, pDesc: *const DXGI_SURFACE_DESC, NumSurfaces: UINT, Usage: DXGI_USAGE,
        pSharedResource: *const DXGI_SHARED_RESOURCE, ppSurface: *mut *mut IDXGISurface
    ) -> HRESULT,
    fn QueryResourceResidency(
        &self, ppResources: *const *mut IUnknown, pResidencyStatus: *mut DXGI_RESIDENCY,
        NumResources: UINT
    ) -> HRESULT,
    fn SetGPUThreadPriority(&self, Priority: INT) -> HRESULT,
    fn GetGPUThreadPriority(&self, pPriority: *mut INT) -> HRESULT
});
ENUM!{enum DXGI_ADAPTER_FLAG {
    DXGI_ADAPTER_FLAG_NONE,
    DXGI_ADAPTER_FLAG_REMOTE,
    DXGI_ADAPTER_FLAG_SOFTWARE,
}}
STRUCT!{struct DXGI_ADAPTER_DESC1 {
    Description: [WCHAR; 128],
    VendorId: UINT,
    DeviceId: UINT,
    SubSysId: UINT,
    Revision: UINT,
    DedicatedVideoMemory: SIZE_T,
    DedicatedSystemMemory: SIZE_T,
    SharedSystemMemory: SIZE_T,
    AdapterLuid: LUID,
    Flags: UINT,
}}
STRUCT!{struct DXGI_DISPLAY_COLOR_SPACE {
    PrimaryCoordinates: [[FLOAT; 2]; 8],
    WhitePoints: [[FLOAT; 2]; 16],
}}
RIDL!(
interface IDXGIFactory1(IDXGIFactory1Vtbl): IDXGIFactory(IDXGIFactoryVtbl) {
    fn EnumAdapters1(&self, Adapter: UINT, ppAdapter: *mut *mut IDXGIAdapter1) -> HRESULT,
    fn IsCurrent(&self) -> BOOL
});
RIDL!(
interface IDXGIAdapter1(IDXGIAdapter1Vtbl): IDXGIAdapter(IDXGIAdapterVtbl) {
    fn GetDesc1(&self, pDesc: *mut DXGI_ADAPTER_DESC1) -> HRESULT
});
RIDL!(
interface IDXGIDevice1(IDXGIDevice1Vtbl): IDXGIDevice(IDXGIDeviceVtbl) {
    fn SetMaximumFrameLatency(&self, MaxLatency: UINT) -> HRESULT,
    fn GetMaximumFrameLatency(&self, pMaxLatency: *mut UINT) -> HRESULT
});
