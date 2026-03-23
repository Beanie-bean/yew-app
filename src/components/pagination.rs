use yew::{prelude::*};

#[derive(Properties, Clone, PartialEq)]
pub struct Page {
    pub current: u32,
    pub on_click: Callback<u32>
}

#[hook]
pub fn use_pagination(page: &Page) -> (u32, Callback<u32>) {
    let Page { current, on_click } = page.clone();
    let current_page = use_state(|| current);

    let set_page = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| {
            if page > 0 {
                current_page.set(page);
                on_click.emit(page);
            }
        })
    };
    (*current_page, set_page)
}


#[component]
pub fn Pagination(page: &Page) -> Html {
    let (current_page, set_page) = use_pagination(&Page {
        current: page.current,
        on_click: page.on_click.clone()
    });

    html! {
        <nav>
            <ul class="pagination justify-content-center">
                if current_page <= 1 {
                    <li class="page-item">
                        <button class="page-link bi bi-skip-backward disabled"></button>
                    </li>
                    <li class="page-item">
                        <button class="page-link disabled">{"Previous"}</button>
                    </li>
                }
                else {
                    <li class="page-item">
                        <button 
                            onclick={{
                                let set_page = set_page.clone();
                                Callback::from(move |_| {
                                    set_page.emit(1)
                                })
                            }}class="page-link bi bi-skip-backward">
                        </button>
                    </li>
                    <li class="page-item">
                        <button 
                            onclick={{
                                let set_page = set_page.clone();
                                Callback::from(move |_| {
                                    set_page.emit(current_page.clone() - 1)
                                })
                            }}class="page-link">{"Previous"}
                        </button>
                    </li>
                }
                <li class="page-item">
                    <button class="page-link">{current_page}</button>
                </li>
                <li class="page-item">
                    <button 
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(current_page.clone() + 1)
                            })
                        }} class="page-link">{"Next"}
                    </button>
                </li>
            </ul>
        </nav>
    }
}