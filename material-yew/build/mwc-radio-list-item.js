import{_ as t,q as e,p as s,P as i,h as r,c as o,f as c,Q as h,U as a,d as l}from"./core.js";import"./mwc-radio.js";class d extends i{constructor(){super(...arguments),this.left=!1,this.graphic="control",this._changeFromClick=!1}render(){const t={"mdc-list-item__graphic":this.left,"mdc-list-item__meta":!this.left},e=this.renderText(),s=this.graphic&&"control"!==this.graphic&&!this.left?this.renderGraphic():r``,i=this.hasMeta&&this.left?this.renderMeta():r``,h=this.renderRipple();return r`
      ${h}
      ${s}
      ${this.left?"":e}
      <mwc-radio
          global
          class=${o(t)}
          tabindex=${this.tabindex}
          name=${c(null===this.group?void 0:this.group)}
          .checked=${this.selected}
          ?disabled=${this.disabled}
          @checked=${this.onChange}>
      </mwc-radio>
      ${this.left?e:""}
      ${i}`}onClick(){this._changeFromClick=!0,super.onClick()}async onChange(t){const e=t.target;this.selected===e.checked||(this._skipPropRequest=!0,this.selected=e.checked,await this.updateComplete,this._skipPropRequest=!1,this._changeFromClick||this.fireRequestSelected(this.selected,"interaction")),this._changeFromClick=!1}}t([e("slot")],d.prototype,"slotElement",void 0),t([e("mwc-radio")],d.prototype,"radioElement",void 0),t([s({type:Boolean})],d.prototype,"left",void 0),t([s({type:String,reflect:!0})],d.prototype,"graphic",void 0);let n=class extends d{};n.styles=[h,a],n=t([l("mwc-radio-list-item")],n);export{n as RadioListItem};
