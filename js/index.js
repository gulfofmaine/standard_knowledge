import init, { StandardsLibrary } from "./pkg/standard_knowledge_js.js"

// Need to initialize WASM before we can use the library
await init();


class Standard extends HTMLElement {
    #standard
    #show = false

    set standard(standard) {
        this.#standard = standard;
        this.update();
    }

    update() {
        let attrs = Array.from(this.#standard.attrs());

        const mapToObj = (m) => {
            return Array.from(m).reduce((obj, [key, value]) => {
                obj[key] = value;
                return obj
            }, {})
        }

        this.innerHTML = `
            <div class="accordion-item">
                <h4 class="accordion-header bg-primary-subtle">
                ${this.#standard.name}
                </h4>
                <div id="collapse${this.#standard.name}" class="accordion-collapse show" aria-labelledby="heading${this.#standard.name}" data-bs-parent="#accordionExample">
                    <div class="accordion-body">
                        <table class="table">
                            <tr class="table-primary">
                                <td>Unit</td>
                                <td>${this.#standard.unit}</td>
                            </tr>
                            <tr class="table-primary">
                                <td>Aliases</td>
                                <td>${this.#standard.aliases.join(", ")}</td>
                            </tr>
                            <tr title="A more human readable name for the standard">
                                <td>Long Name</td>
                                <td>${this.#standard.longName}</td>
                            </tr>
                            <tr title="Category of measurement for the Integrated Ocean Observing System">
                                <td>IOOS Category</td>
                                <td>${this.#standard.ioosCategory}</td>
                            </tr>
                            <tr title="When the standard name isn't used for a column or variable, what might commonly get used instead.">
                                <td>Common variable names</td>
                                <td>${this.#standard.commonVariableNames.join(", ")}</td>
                            </tr>
                            <tr title="Standards that are usually used together">
                                <td>Sibling Standards</td>
                                <td>${this.#standard.siblingStandards.join(", ")}</td>
                            </tr>
                            <tr title="Standards that measure generally similar things, but differ in specifics that are worth investigating.">
                                <td>Related standards</td>
                                <td>${this.#standard.relatedStandards.join(", ")}</td>
                            </tr>
                            <tr title="Other units that may be used rather than the one defined in the standard.">
                                <td>Other Units</td>
                                <td>${this.#standard.otherUnits.join(", ")}</td>
                            </tr>
                        </table>

                        <div class="bg-primary-subtle">
                        <h4>Description:</h4>
                        <p>${this.#standard.description}</p>
                        </div>

                        ${attrs.length === 0 ? "<p>No attributes</p>" : `
                            <h4>Suggested attributes:</h4>
                            <code>
                                ${JSON.stringify(mapToObj(this.#standard.attrs()), null, 2)}
                            </code>
                        `}

                        ${this.#standard.comments ? `
                            <h4>Comments:</h4>
                            <p>${this.#standard.comments}</p>
                        ` : ""}

                        ${this.#standard.qartod.length > 0 ? `
                            <h4>QARTOD Tests:</h4>
                            <ul>
                                ${this.#standard.qartod.map(q => `<li>${q.name} (<code>${q.slug}</code>) <p>${q.description}</p></li>`).join("")}
                            </ul>
                        ` : ""}
                    </div>
                </div>
            </div>
        `;
    }
}

customElements.define("x-standard", Standard);

class GetStandard extends HTMLElement {
    connectedCallback() {
        this.innerHTML = `
        <div class="card">
            <div class="card-header">
                <h2>Get knowledge about a standard</h2>
                <form>
                    <input type="text" id="name" name="name" placeholder="Enter a CF standard name" />
                    <button type="submit">Show Standard</button>
                </form>
            </div>
            <div class="card-body" id="result">
                Please enter a standard
            </div>
        </div>
        `

        this.querySelector("form").onsubmit = (e) => {
            e.preventDefault();
            const data = new FormData(e.target)
            const name = data.get("name").toString().trim();

            if (name) {
                try {
                    let standard = this.library.get(name); // Just to see if it exists
                    this.querySelector("#result").innerHTML = `
                        <x-standard></x-standard>
                    `;
                    this.querySelector("x-standard").standard = standard;
                } catch (error) {
                    this.querySelector("#result").innerHTML = `
                    <div class="alert alert-danger" role="alert">
                        Could not find standard with name <strong>${name}</strong>
                    </div>
                    `
                    console.error(error);
                }
            }
        }
    }
}

customElements.define("x-get-standard", GetStandard);

class App extends HTMLElement {
    connectedCallback() {
        this.textContent = "Invalid standard"

        this.library = new StandardsLibrary();
        this.library.loadCfStandards();
        this.library.loadKnowledge();
        this.library.loadTestSuites();

        this.innerHTML = `
        <x-get-standard></x-get-standard>
        `;

        this.querySelector("x-get-standard").library = this.library;

        // try {
        //     // let standard = this.library.get("air_temperature")
        //     // let standard = this.library.get("air_pressure_at_mean_sea_level")
        //     let standard = this.library.get("sea_surface_height_above_geopotential_datum")
        //     // console.log(standard.attrs())

        //     // this.querySelector("x-standard").standard = standard;
        //     this.querySelector("x-standard").show = true;
        // } catch (error) {
        //     console.error(error)
        // }
    }
}

customElements.define("x-app", App);




// const app = () => {
//     customElements.define("x-standard", Standard);
//     customElements.define("x-app", App);
// }

// document.addEventListener("DOMContentLoaded", app);
