use yew::prelude::*;

use crate::components::{
    footer::Footer, navbar::Navbar, purchase::purchases_list::PurchasesList,
    purchase::seller_summary::SellerSummary,
};

#[function_component(PurchasesPage)]
pub fn purchases_page() -> Html {
    html! {
        <>
            <Navbar />
            <main>
              <PurchasesList />
              <SellerSummary />
            </main>
            <Footer />
        </>
    }
}
