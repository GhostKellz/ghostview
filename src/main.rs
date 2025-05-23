slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "👻 GhostView";
            font-size: 32px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
