use napi_derive::napi;

// The two `From` impls below `match` exhaustively over every variant in both
// directions, so any drift between this list and `simulang_rs::AriaRole` is a
// compile error — provided the upstream enum stays non-`#[non_exhaustive]`.
macro_rules! aria_roles {
  ($($variant:ident),* $(,)?) => {
    #[napi]
    /// Cross-platform ARIA / Playwright role. Used as the `role` field on
    /// accessibility snapshots and as the search key in
    /// [`AccessibilityTree::find`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum AriaRole {
      $($variant,)*
    }

    impl From<simulang_rs::AriaRole> for AriaRole {
      fn from(role: simulang_rs::AriaRole) -> Self {
        match role {
          $(simulang_rs::AriaRole::$variant => AriaRole::$variant,)*
        }
      }
    }

    impl From<AriaRole> for simulang_rs::AriaRole {
      fn from(role: AriaRole) -> Self {
        match role {
          $(AriaRole::$variant => simulang_rs::AriaRole::$variant,)*
        }
      }
    }
  };
}

/// Lowercase ARIA name for a role — matches the form used in
/// snapshot output and accepted by `AccessibilityTree.find()`.
///
/// `AriaRole` is exposed to JS as a numeric enum, so the usual
/// TypeScript reverse-mapping trick (`AriaRole[role]`) does not
/// work; use this function instead.
#[napi]
#[must_use]
pub fn aria_role_to_string(role: AriaRole) -> &'static str {
  simulang_rs::AriaRole::from(role).as_str()
}

aria_roles!(
  Alert,
  AlertDialog,
  Application,
  Article,
  Audio,
  Banner,
  Blockquote,
  Button,
  Caption,
  Cell,
  Checkbox,
  Code,
  ColumnHeader,
  Combobox,
  Complementary,
  ContentInfo,
  Definition,
  Deletion,
  Dialog,
  Directory,
  Document,
  Emphasis,
  Feed,
  Figure,
  Form,
  Generic,
  Grid,
  GridCell,
  Group,
  Heading,
  Img,
  Insertion,
  Link,
  List,
  ListItem,
  Listbox,
  Log,
  Main,
  Mark,
  Marquee,
  Math,
  Menu,
  MenuBar,
  MenuItem,
  MenuItemCheckbox,
  MenuItemRadio,
  Meter,
  Navigation,
  Note,
  Option,
  Paragraph,
  Password,
  Presentation,
  ProgressBar,
  Radio,
  RadioGroup,
  Region,
  Row,
  RowGroup,
  RowHeader,
  Scrollbar,
  Search,
  Searchbox,
  Separator,
  Slider,
  SpinButton,
  Status,
  Strong,
  Subscript,
  Suggestion,
  Superscript,
  Switch,
  Tab,
  Table,
  TabList,
  TabPanel,
  Term,
  Text,
  Textbox,
  Time,
  Timer,
  Toolbar,
  Tooltip,
  Tree,
  TreeItem,
  Treegrid,
  Video,
  Window,
);
