
#[macro_export]
macro_rules! widget {
    ($widget_trait:ident<$view_types_generic:ident> {$(
        $field_name:ident : $field_type:ty
    ),*}) => {

        paste::paste!{
            let [<$widget_trait Public>]<$view_types_generic>
                = hidden::WidgetPublic<$view_types_generic>;
            let [<$widget_trait Private>]<$view_types_generic>
                = hidden::WidgetPrivate<$view_types_generic>;
        }

        mod hidden {
            use crate::view::{WidgetSelector, AnyConsumer};
            use super::*;

            pub struct WidgetPublic<T> {
                selector: WidgetSelector<WidgetPrivate<T>>,
                sender: Arc<Sender<AnyConsumer>>,
            }

            pub struct WidgetPrivate<T: ViewTypes> {
                public: ProgressBarPublic<T>,
                $(
                    $field_name : $field_type
                ),*
            }
        }

    };
}
