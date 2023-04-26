use windows::{
    core::*, Win32::Foundation::*, Win32::System::Registry::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

/// Tests a few APIs that have reserved parameters to ensure they can be called with `None`.
#[test]
fn test() -> Result<()> {
    unsafe {
        assert_eq!(InSendMessageEx(None), ISMEX_NOSEND);
        assert!(CreateThreadpool(None).0 != 0);

        TrackPopupMenu(
            HMENU(0),
            TPM_LEFTBUTTON,
            1,
            2,
            0,
            HWND(0),
            Default::default(),
        )
        .unwrap_err();

        let mut key = HKEY::default();
        RegOpenKeyExA(HKEY_CLASSES_ROOT, s!(r".txt"), 0, KEY_QUERY_VALUE, &mut key)?;
        let mut len = 0;
        RegQueryValueExA(key, s!("Content Type"), None, None, None, Some(&mut len))?;
        let mut buffer = vec![0u8; (len) as usize];
        RegQueryValueExA(
            key,
            s!("Content Type"),
            None,
            None,
            Some(buffer.as_mut_ptr() as _),
            Some(&mut len),
        )?;
        assert_eq!(String::from_utf8_lossy(&buffer), "text/plain\0");
        Ok(())
    }
}
