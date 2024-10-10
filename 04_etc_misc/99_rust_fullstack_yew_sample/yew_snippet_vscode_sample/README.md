# yew(Rust) snippet Setting(VSCode)

https://yew.rs/docs/getting-started/editor-setup

- Snippets:Configure User
- New Snippet files for

Navigate to File > Preferences > User Snippets.
Select Rust as the language.
Add the following snippet in the snippet JSON file:

- prefix `yewfc` , prefix`yewsc`

```
{
    "New Yew function component": {
        "prefix": "yewfc",
        "body": [
            "#[derive(PartialEq, Properties)]",
            "pub struct ${1:ComponentName}Props {}",
            "",
            "#[function_component]",
            "pub fn $1(props: &${1}Props) -> Html {",
            "    let ${1}Props {} = props;",
            "    html! {",
            "        <${2:div}>$0</${2}>",
            "    }",
            "}"
        ],
        "description": "Create a minimal Yew function component"
    },
    "New Yew struct component": {
        "prefix": "yewsc",
        "body": [
            "pub struct ${1:ComponentName};",
            "",
            "pub enum ${1}Msg {",
            "}",
            "",
            "impl Component for ${1} {",
            "    type Message = ${1}Msg;",
            "    type Properties = ();",
            "",
            "    fn create(ctx: &Context<Self>) -> Self {",
            "        Self",
            "    }",
            "",
            "    fn view(&self, ctx: &Context<Self>) -> Html {",
            "        html! {",
            "            $0",
            "        }",
            "    }",
            "}"
        ],
        "description": "Create a new Yew component with a message enum"
    }
}
```

# VS Code

## Rust-Yew extension

https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew

This is a work in progress, and community maintained project! Please see details and direct related bug reports / issues / questions over to the extension's repository

Rust-Yew extension is avaliable on VSC Marketplace, providing syntax highlight, renames, hover, and more.

Emmet support should work out of the box, if not please fall back to edditing the settings.json file:

```
"emmet.includeLanguages": {
    "rust": "html",
}
```
