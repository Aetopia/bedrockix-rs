use std::sync::LazyLock;

use windows::{
    ApplicationModel::Package,
    System::AppDiagnosticInfo,
    Win32::UI::Shell::{
        AO_NOERRORUI, ApplicationActivationManager, IApplicationActivationManager,
        IPackageDebugSettings, PackageDebugSettings,
    },
    core::{HSTRING, Result},
};

use super::COM;

static MANAGER: LazyLock<COM<IApplicationActivationManager>> =
    LazyLock::new(|| COM::create(&ApplicationActivationManager).unwrap());

static SETTINGS: LazyLock<COM<IPackageDebugSettings>> =
    LazyLock::new(|| COM::create(&PackageDebugSettings).unwrap());

pub struct App {
    value: AppDiagnosticInfo,
}

impl App {
    pub fn new(value: &str) -> Result<Self> {
        Ok(Self {
            value: AppDiagnosticInfo::RequestInfoForAppUserModelId(&HSTRING::from(value))?
                .get()?
                .First()?
                .Current()?,
        })
    }

    pub fn launch(&self) -> Result<u32> {
        unsafe {
            MANAGER.ActivateApplication(
                &self.value.AppInfo()?.AppUserModelId()?,
                None,
                AO_NOERRORUI,
            )
        }
    }

    pub fn running(&self) -> Result<bool> {
        Ok(self.value.GetResourceGroups().iter().any(|value| {
            value
                .into_iter()
                .any(|value| match value.GetMemoryReport() {
                    Ok(value) => value.PrivateCommitUsage().unwrap_or(0) > 0,
                    Err(_) => false,
                })
        }))
    }

    pub fn terminate(&self) -> Result<()> {
        unsafe { SETTINGS.TerminateAllProcesses(&self.package()?.Id()?.FullName()?) }
    }

    pub fn package(&self) -> Result<Package> {
        Ok(self.value.AppInfo()?.Package()?)
    }

    pub fn debug(&self, value: bool) -> Result<()> {
        let package = &self.package()?.Id()?.FullName()?;
        unsafe {
            if value {
                SETTINGS.EnableDebugging(package, None, None)
            } else {
                SETTINGS.DisableDebugging(package)
            }
        }
    }
}
