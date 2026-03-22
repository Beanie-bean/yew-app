use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct List {
    id: usize,
    name: AttrValue,
    desc: AttrValue
}

#[derive(Properties, PartialEq)]
struct MyListsListProps {
    my_lists: Vec<List>
}

#[component]
fn MyListsList(MyListsListProps { my_lists }: &MyListsListProps) -> Html {
    html! {
        for list in my_lists {
            <p key={list.id}>{format!("{} {}", list.name, list.desc)}</p>
        }
    }
}

#[component]
pub fn MyLists() -> Html {

    let my_lists = vec![
        List {
            id: 1,
            name: "Lista".into(),
            desc: "kuvaus".into(),
        },
    ];
    html! {
        <>
            <h1 class="d-flex justify-content-center">{"All Games"}</h1>
            <div class="d-flex justify-content-center">
                <div class="w-50">
                    <table class="table table-bordered">
                        <thead>
                            <tr>
                                <th>{"Name"}</th>
                                <th width="15%">{"Release Year"}</th>
                                <th width="10%"></th>
                            </tr>
                        </thead>
                        <tbody>
                        </tbody>
                    </table>
                </div>
            </div>
        </>
    }
}


