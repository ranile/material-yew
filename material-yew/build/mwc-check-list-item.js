import{_ as e,i as t,e as s,O as c,y as i,o as h,N as r,P as o,f as a}from"./core.js";import"./mwc-checkbox.js";class d extends c{constructor(){super(...arguments),this.left=!1,this.graphic="control"}render(){const e={"mdc-deprecated-list-item__graphic":this.left,"mdc-deprecated-list-item__meta":!this.left},t=this.renderText(),s=this.graphic&&"control"!==this.graphic&&!this.left?this.renderGraphic():i``,c=this.hasMeta&&this.left?this.renderMeta():i``,r=this.renderRipple();return i`
      ${r}
      ${s}
      ${this.left?"":t}
      <span class=${h(e)}>
        <mwc-checkbox
            reducedTouchTarget
            tabindex=${this.tabindex}
            .checked=${this.selected}
            ?disabled=${this.disabled}
            @change=${this.onChange}>
        </mwc-checkbox>
      </span>
      ${this.left?t:""}
      ${c}`}async onChange(e){const t=e.target;this.selected===t.checked||(this._skipPropRequest=!0,this.selected=t.checked,await this.updateComplete,this._skipPropRequest=!1)}}e([t("slot")],d.prototype,"slotElement",void 0),e([t("mwc-checkbox")],d.prototype,"checkboxElement",void 0),e([s({type:Boolean})],d.prototype,"left",void 0),e([s({type:String,reflect:!0})],d.prototype,"graphic",void 0);let p=class extends d{};p.styles=[r,o],p=e([a("mwc-check-list-item")],p);export{p as CheckListItem};
