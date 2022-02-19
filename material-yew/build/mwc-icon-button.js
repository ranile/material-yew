import{_ as e,e as t,a as s,i,b as p,t as l,c as a,s as o,R as r,$ as n,l as d,m as h,n as u}from"./core.js";class c extends o{constructor(){super(...arguments),this.disabled=!1,this.icon="",this.shouldRenderRipple=!1,this.rippleHandlers=new r(()=>(this.shouldRenderRipple=!0,this.ripple))}renderRipple(){return this.shouldRenderRipple?n`
            <mwc-ripple
                .disabled="${this.disabled}"
                unbounded>
            </mwc-ripple>`:""}focus(){const e=this.buttonElement;e&&(this.rippleHandlers.startFocus(),e.focus())}blur(){const e=this.buttonElement;e&&(this.rippleHandlers.endFocus(),e.blur())}render(){return n`<button
        class="mdc-icon-button mdc-icon-button--display-flex"
        aria-label="${this.ariaLabel||this.icon}"
        aria-haspopup="${d(this.ariaHasPopup)}"
        ?disabled="${this.disabled}"
        @focus="${this.handleRippleFocus}"
        @blur="${this.handleRippleBlur}"
        @mousedown="${this.handleRippleMouseDown}"
        @mouseenter="${this.handleRippleMouseEnter}"
        @mouseleave="${this.handleRippleMouseLeave}"
        @touchstart="${this.handleRippleTouchStart}"
        @touchend="${this.handleRippleDeactivate}"
        @touchcancel="${this.handleRippleDeactivate}"
    >${this.renderRipple()}
    <i class="material-icons">${this.icon}</i>
    <span
      ><slot></slot
    ></span>
  </button>`}handleRippleMouseDown(e){const t=()=>{window.removeEventListener("mouseup",t),this.handleRippleDeactivate()};window.addEventListener("mouseup",t),this.rippleHandlers.startPress(e)}handleRippleTouchStart(e){this.rippleHandlers.startPress(e)}handleRippleDeactivate(){this.rippleHandlers.endPress()}handleRippleMouseEnter(){this.rippleHandlers.startHover()}handleRippleMouseLeave(){this.rippleHandlers.endHover()}handleRippleFocus(){this.rippleHandlers.startFocus()}handleRippleBlur(){this.rippleHandlers.endFocus()}}e([t({type:Boolean,reflect:!0})],c.prototype,"disabled",void 0),e([t({type:String})],c.prototype,"icon",void 0),e([s,t({type:String,attribute:"aria-label"})],c.prototype,"ariaLabel",void 0),e([s,t({type:String,attribute:"aria-haspopup"})],c.prototype,"ariaHasPopup",void 0),e([i("button")],c.prototype,"buttonElement",void 0),e([p("mwc-ripple")],c.prototype,"ripple",void 0),e([l()],c.prototype,"shouldRenderRipple",void 0),e([a({passive:!0})],c.prototype,"handleRippleMouseDown",null),e([a({passive:!0})],c.prototype,"handleRippleTouchStart",null);let R=class extends c{};R.styles=[h],R=e([u("mwc-icon-button")],R);export{R as IconButton};
