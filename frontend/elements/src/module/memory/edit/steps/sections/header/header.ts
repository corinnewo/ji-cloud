import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import "@elements/module/_common/header";

const STR_TITLE = "Create a Memory Game";
@customElement('header-memory')
export class _ extends LitElement {
  static get styles() {
      return [css`

    `];
  }

  render() {
      return html`
          <module-header slot="header" title="${STR_TITLE}">
            <empty-fragment slot="controller">
                <slot name="controller"></slot>
            </empty-fragment>
            <slot name="button"></slot>
          </module-header>
      `;
  }
}
