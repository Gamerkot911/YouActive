fn main() {
    windows::build! {
        Windows::ApplicationModel::Activation::*,
        Windows::UI::Xaml::Controls::TextBlock,
        Windows::UI::Xaml::*,
    };
}