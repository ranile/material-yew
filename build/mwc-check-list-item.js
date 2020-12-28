import{_ as e,q as t,p as s,P as i,h as c,c as h,Q as o,U as r,d as a}from"./core.js";import"./mwc-checkbox.js";class l extends i{constructor(){super(...arguments),this.left=!1,this.graphic="control"}render(){const e={"mdc-list-item__graphic":this.left,"mdc-list-item__meta":!this.left},t=this.renderText(),s=this.graphic&&"control"!==this.graphic&&!this.left?this.renderGraphic():c``,i=this.hasMeta&&this.left?this.renderMeta():c``,o=this.renderRipple();return c`
      ${o}
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
      ${i}`}async onChange(e){const t=e.target;this.selected===t.checked||(this._skipPropRequest=!0,this.selected=t.checked,await this.updateComplete,this._skipPropRequest=!1)}}e([t("slot")],l.prototype,"slotElement",void 0),e([t("mwc-checkbox")],l.prototype,"checkboxElement",void 0),e([s({type:Boolean})],l.prototype,"left",void 0),e([s({type:String,reflect:!0})],l.prototype,"graphic",void 0);let p=class extends l{};p.styles=[o,r],p=e([a("mwc-check-list-item")],p);export{p as CheckListItem};
