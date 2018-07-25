macro_rules! generate_events {
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

generate_events!(EnchantronEvent, StartGame{ pub new: bool });


