import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("footer-contact")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        p {
          color: #5590fc;
        }
      `,
    ];
  }

  render() {
    const STR_CONTACT = "If you need our help, kindly contact us at: ";
    return html`
      <p>
      ${STR_CONTACT} &nbsp;
        <a href="mailto:support@jigzi.org">support@jigzi.org</a>
      </p>
    `;
  }
}
