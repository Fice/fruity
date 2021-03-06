use crate::core::FourCharCode;
use std::fmt;

/// Specifies the event class of an Apple event.
///
/// Apple events are identified by their event class and event ID attributes.
/// The event class is the attribute that identifies a group of related Apple
/// events. When you call the `AEProcessAppleEvent` function, the Apple Event
/// Manager uses these attributes to identify a handler for a specific Apple
/// event.
///
/// See [documentation](https://developer.apple.com/documentation/coreservices/aeeventclass?language=objc).
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AEEventClass(pub FourCharCode);

impl fmt::Debug for AEEventClass {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format as escaped ASCII string.
        self.0.fmt(f)
    }
}

impl AEEventClass {
    /// Returns an instance from the integer value.
    #[inline]
    pub const fn from_int(int: u32) -> Self {
        Self(FourCharCode::from_int(int))
    }

    /// Returns an instance from the 4-character code.
    #[inline]
    pub const fn from_chars(chars: [u8; 4]) -> Self {
        Self(FourCharCode::from_chars(chars))
    }

    /// Returns this descriptor's integer value.
    #[inline]
    pub const fn into_int(self) -> u32 {
        self.0.into_int()
    }

    /// Returns this descriptor's 4-character code.
    #[inline]
    pub const fn into_chars(self) -> [u8; 4] {
        self.0.into_chars()
    }
}

impl AEEventClass {
    // Despite this being defined via `CF_ENUM(DescType)`, it is an event class.

    /// Value: `aevt`.
    ///
    /// An Apple event sent by the Mac OS; applications that present a graphical
    /// interface to the user should be able to any events sent by the Mac OS
    /// that apply to the application.
    ///
    /// See [`AEEventID` constants](struct.AEEventID.html#kCoreEventClass).
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kcoreeventclass?language=objc).
    #[doc(alias = "kCoreEventClass")]
    pub const CORE: Self = Self::from_chars(*b"aevt");
}

/// Events in HIToolbox framework.
impl AEEventClass {
    /// Events related to the mouse (mouse down/up/moved).
    ///
    /// Value: `mous`.
    #[doc(alias = "kEventClassMouse")]
    pub const MOUSE: Self = Self::from_chars(*b"mous");

    /// Events related to the keyboard.
    ///
    /// Value: `keyb`.
    #[doc(alias = "kEventClassKeyboard")]
    pub const KEYBOARD: Self = Self::from_chars(*b"keyb");

    /// Events related to text input (by keyboard, or by input method).
    ///
    /// Value: `text`.
    #[doc(alias = "kEventClassTextInput")]
    pub const TEXT_INPUT: Self = Self::from_chars(*b"text");

    /// Application-level events (launch, quit, etc.).
    ///
    /// Value: `appl`.
    #[doc(alias = "kEventClassApplication")]
    pub const APPLICATION: Self = Self::from_chars(*b"appl");

    /// Apple Events.
    ///
    /// Value: `eppc`.
    #[doc(alias = "kEventClassAppleEvent")]
    pub const APPLE_EVENT: Self = Self::from_chars(*b"eppc");

    /// Events related to menus.
    ///
    /// Value: `menu`.
    #[doc(alias = "kEventClassMenu")]
    pub const MENU: Self = Self::from_chars(*b"menu");

    /// Events related to windows.
    ///
    /// Value: `wind`.
    #[doc(alias = "kEventClassWindow")]
    pub const WINDOW: Self = Self::from_chars(*b"wind");

    /// Events related to controls.
    ///
    /// Value: `cntl`.
    #[doc(alias = "kEventClassControl")]
    pub const CONTROL: Self = Self::from_chars(*b"cntl");

    /// Events related to commands generated by menu items or controls.
    ///
    /// These events contain `HICommand` structures.
    ///
    /// Value: `cmds`.
    #[doc(alias = "kEventClassCommand")]
    pub const COMMAND: Self = Self::from_chars(*b"cmds");

    /// Events related to tablets.
    ///
    /// Value: `tblt`.
    #[doc(alias = "kEventClassTablet")]
    pub const TABLET: Self = Self::from_chars(*b"tblt");

    /// Events related to File Manager volumes.
    ///
    /// Value: `vol `.
    #[doc(alias = "kEventClassVolume")]
    pub const VOLUME: Self = Self::from_chars(*b"vol ");

    /// Events related to the Appearance Manager.
    ///
    /// Value: `appm`.
    #[doc(alias = "kEventClassAppearance")]
    pub const APPEARANCE_MANAGER: Self = Self::from_chars(*b"appm");

    /// Events related to the Services Manager.
    ///
    /// Value: `serv`.
    #[doc(alias = "kEventClassService")]
    pub const SERVICE: Self = Self::from_chars(*b"serv");

    /// Events related to toolbars.
    ///
    /// Value: `tbar`.
    #[doc(alias = "kEventClassToolbar")]
    pub const TOOLBAR: Self = Self::from_chars(*b"tbar");

    /// Events related to toolbar items.
    ///
    /// Value: `tbit`.
    #[doc(alias = "kEventClassToolbarItem")]
    pub const TOOLBAR_ITEM: Self = Self::from_chars(*b"tbit");

    /// Events related to toolbar item views.
    ///
    /// Value: `tbiv`.
    #[doc(alias = "kEventClassToolbarItemView")]
    pub const TOOLBAR_ITEM_VIEW: Self = Self::from_chars(*b"tbiv");

    /// Events related to application accessibility.
    ///
    /// Value: `acce`.
    #[doc(alias = "kEventClassAccessibility")]
    pub const ACCESSIBILITY: Self = Self::from_chars(*b"acce");

    /// Events related to the system.
    ///
    /// Value: `macs`.
    #[doc(alias = "kEventClassSystem")]
    pub const SYSTEM: Self = Self::from_chars(*b"macs");

    /// Events related to ink.
    ///
    /// Value: `ink `.
    #[doc(alias = "kEventClassInk")]
    pub const INK: Self = Self::from_chars(*b"ink "); /* DEPRECATED on macOS 10.14 and later. */

    /// Value: `tdac`.
    #[doc(alias = "kEventClassTSMDocumentAccess")]
    pub const TSM_DOCUMENT_ACCESS: Self = Self::from_chars(*b"tdac");

    /// Events related to gestures: magnify, swipe, rotate.
    ///
    /// Value: `gest`.
    #[doc(alias = "kEventClassGesture")]
    pub const GESTURE: Self = Self::from_chars(*b"gest");

    /// Value: `cloc`.
    #[doc(alias = "kEventClassClockView")]
    pub const CLOCK_VIEW: Self = Self::from_chars(*b"cloc");

    /// Value: `txfd`.
    #[doc(alias = "kEventClassTextField")]
    pub const TEXT_FIELD: Self = Self::from_chars(*b"txfd");

    /// Value: `hiob`.
    #[doc(alias = "kEventClassHIObject")]
    pub const HI_OBJECT: Self = Self::from_chars(*b"hiob");

    /// Value: `dele`.
    #[doc(alias = "kEventClassDelegate")]
    pub const DELEGATE: Self = Self::from_chars(*b"dele");

    /// Value: `scrl`.
    #[doc(alias = "kEventClassScrollable")]
    pub const SCROLLABLE: Self = Self::from_chars(*b"scrl");

    /// Value: `hicb`.
    #[doc(alias = "kEventClassHIComboBox")]
    pub const HI_COMBO_BOX: Self = Self::from_chars(*b"hicb");

    /// Value: `srfd`.
    #[doc(alias = "kEventClassSearchField")]
    pub const SEARCH_FIELD: Self = Self::from_chars(*b"srfd");

    /// Value: `appr`.
    #[doc(alias = "kAppearanceEventClass")]
    pub const APPEARANCE: Self = Self::from_chars(*b"appr");

    /// Value: `hidb`.
    #[doc(alias = "kEventClassDataBrowser")]
    pub const DATA_BROWSER: Self = Self::from_chars(*b"hidb");
}

/// Events in HIServices framework.
impl AEEventClass {
    /// Value: `GURL`.
    #[doc(alias = "kInternetEventClass")]
    pub const INTERNET: Self = Self::from_chars(*b"GURL");

    /// Edit preference in Internet configuration.
    ///
    /// Value: `ICAp`.
    #[doc(alias = "kICEditPreferenceEventClass")]
    pub const IC_EDIT_PREFERENCE: Self = Self::from_chars(*b"ICAp");
}

/// Events in Open Scripting framework.
impl AEEventClass {
    /// Value: `dhub`.
    #[doc(alias = "kDigiHubEventClass")]
    pub const DIGI_HUB: Self = Self::from_chars(*b"dhub");
}

/// Events in Common Panels framework.
impl AEEventClass {
    /// Value: `font`.
    #[doc(alias = "kEventClassFont")]
    pub const FONT: Self = Self::from_chars(*b"font");
}

/// Events in Address Book framework.
impl AEEventClass {
    /// People-picker events.
    ///
    /// Value: `abpp`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/addressbook/1591603-people-picker_event_class/keventclassabpeoplepicker?language=objc).
    #[doc(alias = "kEventClassABPeoplePicker")]
    pub const AB_PEOPLE_PICKER: Self = Self::from_chars(*b"abpp");
}
