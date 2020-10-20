import{_ as e,p as t,q as s,a as l,i,e as p,L as o,R as n,h as r,l as a,d}from"./core.js";class h extends o{constructor(){super(...arguments),this.disabled=!1,this.icon="",this.label="",this.shouldRenderRipple=!1,this.rippleHandlers=new n(()=>(this.shouldRenderRipple=!0,this.ripple))}renderRipple(){return r`${this.shouldRenderRipple?r`
            <mwc-ripple
                .disabled="${this.disabled}"
                unbounded>
            </mwc-ripple>`:""}`}focus(){const e=this.buttonElement;e&&(this.rippleHandlers.startFocus(),e.focus())}blur(){const e=this.buttonElement;e&&(this.rippleHandlers.endFocus(),e.blur())}render(){return r`<button
        class="mdc-icon-button"
        aria-label="${this.label||this.icon}"
        ?disabled="${this.disabled}"
        @focus="${this.handleRippleFocus}"
        @blur="${this.handleRippleBlur}"
        @mousedown="${this.handleRippleMouseDown}"
        @mouseenter="${this.handleRippleMouseEnter}"
        @mouseleave="${this.handleRippleMouseLeave}"
        @touchstart="${this.handleRippleTouchStart}"
        @touchend="${this.handleRippleDeactivate}"
        @touchcancel="${this.handleRippleDeactivate}">
      ${this.renderRipple()}
    <i class="material-icons">${this.icon}</i>
    <slot></slot>
  </button>`}handleRippleMouseDown(e){const t=()=>{window.removeEventListener("mouseup",t),this.handleRippleDeactivate()};window.addEventListener("mouseup",t),this.rippleHandlers.startPress(e)}handleRippleTouchStart(e){this.rippleHandlers.startPress(e)}handleRippleDeactivate(){this.rippleHandlers.endPress()}handleRippleMouseEnter(){this.rippleHandlers.startHover()}handleRippleMouseLeave(){this.rippleHandlers.endHover()}handleRippleFocus(){this.rippleHandlers.startFocus()}handleRippleBlur(){this.rippleHandlers.endFocus()}}e([t({type:Boolean,reflect:!0})],h.prototype,"disabled",void 0),e([t({type:String})],h.prototype,"icon",void 0),e([t({type:String})],h.prototype,"label",void 0),e([s("button")],h.prototype,"buttonElement",void 0),e([l("mwc-ripple")],h.prototype,"ripple",void 0),e([i()],h.prototype,"shouldRenderRipple",void 0),e([p({passive:!0})],h.prototype,"handleRippleMouseDown",null),e([p({passive:!0})],h.prototype,"handleRippleTouchStart",null);let u=class extends h{};u.styles=a,u=e([d("mwc-icon-button")],u);export{u as IconButton};
