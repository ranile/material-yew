import{_ as e,i as t,e as i,O as s,y as r,o as c,l as o,N as h,P as a,f as l}from"./core.js";import"./mwc-radio.js";class d extends s{constructor(){super(...arguments),this.left=!1,this.graphic="control",this._changeFromClick=!1}render(){const e={"mdc-deprecated-list-item__graphic":this.left,"mdc-deprecated-list-item__meta":!this.left},t=this.renderText(),i=this.graphic&&"control"!==this.graphic&&!this.left?this.renderGraphic():r``,s=this.hasMeta&&this.left?this.renderMeta():r``,h=this.renderRipple();return r`
      ${h}
      ${i}
      ${this.left?"":t}
      <mwc-radio
          global
          class=${c(e)}
          tabindex=${this.tabindex}
          name=${o(null===this.group?void 0:this.group)}
          .checked=${this.selected}
          ?disabled=${this.disabled}
          @checked=${this.onChange}>
      </mwc-radio>
      ${this.left?t:""}
      ${s}`}onClick(){this._changeFromClick=!0,super.onClick()}async onChange(e){const t=e.target;this.selected===t.checked||(this._skipPropRequest=!0,this.selected=t.checked,await this.updateComplete,this._skipPropRequest=!1,this._changeFromClick||this.fireRequestSelected(this.selected,"interaction")),this._changeFromClick=!1}}e([t("slot")],d.prototype,"slotElement",void 0),e([t("mwc-radio")],d.prototype,"radioElement",void 0),e([i({type:Boolean})],d.prototype,"left",void 0),e([i({type:String,reflect:!0})],d.prototype,"graphic",void 0);let n=class extends d{};n.styles=[h,a],n=e([l("mwc-radio-list-item")],n);export{n as RadioListItem};
