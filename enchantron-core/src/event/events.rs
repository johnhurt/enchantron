macro_rules! define_events {
  ($events_name:ident, $($e:ident $body:tt ), *) => {

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    pub enum $events_name {
      $(
        $e,
      )*
    }

    $(

    #[derive(Debug, Clone)]
    pub struct $e $body

    impl Into<$events_name> for $e {
      fn into(self) -> $events_name { $events_name::$e }
    }

    )*
  }
}

define_events!(FourFoursEvent,
    LoadResources{},
    StartGame{ pub new: bool },
    Layout{
      pub width: i64,
      pub height: i64,
    },
    Evaluate{}
);


