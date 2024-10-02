use crate::components::*;

use leptos::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Layout id="contact".to_string() aria_label="Contact" class_name="flex-col".to_string()>
            // Flex container to align the heading and button next to each other
            <div class="flex items-center gap-20">
                <h2 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl text-nf-white leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading">
                    <div class="animated-title m-5">
                        <span class="animated-title-element text-nf-white font-regular uppercase">
                            Contact
                        </span>
                    </div>
                    <br/>
                    <div class="animated-title">
                        <span class="animated-title-element text-nf-white font-bold uppercase">
                            Me
                        </span>
                    </div>
                    <div class="animated-title">
                        <a href="/resources/public-key.asc" download class="sm:text-xs md:text-sm hover:text-nf-color m-5 animated-title-element text-nf-white font-regular uppercase transition">
                            (click me for pgp key)
                        </a>
                    </div>
                </h2>

                <Button
                    href="mailto:nicolas.theo.frey@gmail.com".to_string()
                    class_name="mx-16".to_string()
                    label="I can't wait!".to_string()
                />
            </div>

            <div class="grid lg:grid-rows-2 lg:grid-cols-2 lg:grid-flow-col mt-10 md:mt-20">
                <p class="lg:col-span-2 order-1 self-center min-w-full lg:order-3 text-xl md:text-2xl lg:text-3xl leading-p lg:leading-largep text-nf-white font-montserrat">
                    If you are looking for someone in your team, be it indie or at a company: "I'm" always looking for new opportunities.
                </p>
            </div>
        </Layout>
    }
}
