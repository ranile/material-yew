import{_ as e,t,i,a as o,e as s,b as n,c as l,s as a,R as p,y as r,o as d,l as c,z as h,f as u}from"./core.js";class b extends a{constructor(){super(...arguments),this.disabled=!1,this.onIcon="",this.offIcon="",this.on=!1,this.shouldRenderRipple=!1,this.rippleHandlers=new p((()=>(this.shouldRenderRipple=!0,this.ripple)))}handleClick(){this.on=!this.on,this.dispatchEvent(new CustomEvent("icon-button-toggle-change",{detail:{isOn:this.on},bubbles:!0}))}click(){this.mdcRoot.focus(),this.mdcRoot.click()}focus(){this.rippleHandlers.startFocus(),this.mdcRoot.focus()}blur(){this.rippleHandlers.endFocus(),this.mdcRoot.blur()}renderRipple(){return this.shouldRenderRipple?r`
            <mwc-ripple
                .disabled="${this.disabled}"
                unbounded>
            </mwc-ripple>`:""}render(){const e={"mdc-icon-button--on":this.on},t=void 0!==this.ariaLabelOn&&void 0!==this.ariaLabelOff,i=t?void 0:this.on,o=t?this.on?this.ariaLabelOn:this.ariaLabelOff:this.ariaLabel;return r`<button
          class="mdc-icon-button mdc-icon-button--display-flex ${d(e)}"
          aria-pressed="${c(i)}"
          aria-label="${c(o)}"
          @click="${this.handleClick}"
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
        <span class="mdc-icon-button__icon"
          ><slot name="offIcon"
            ><i class="material-icons">${this.offIcon}</i
          ></slot
        ></span>
        <span class="mdc-icon-button__icon mdc-icon-button__icon--on"
          ><slot name="onIcon"
            ><i class="material-icons">${this.onIcon}</i
          ></slot
        ></span>
      </button>`}handleRippleMouseDown(e){const t=()=>{window.removeEventListener("mouseup",t),this.handleRippleDeactivate()};window.addEventListener("mouseup",t),this.rippleHandlers.startPress(e)}handleRippleTouchStart(e){this.rippleHandlers.startPress(e)}handleRippleDeactivate(){this.rippleHandlers.endPress()}handleRippleMouseEnter(){this.rippleHandlers.startHover()}handleRippleMouseLeave(){this.rippleHandlers.endHover()}handleRippleFocus(){this.rippleHandlers.startFocus()}handleRippleBlur(){this.rippleHandlers.endFocus()}}e([i(".mdc-icon-button")],b.prototype,"mdcRoot",void 0),e([o,s({type:String,attribute:"aria-label"})],b.prototype,"ariaLabel",void 0),e([s({type:Boolean,reflect:!0})],b.prototype,"disabled",void 0),e([s({type:String})],b.prototype,"onIcon",void 0),e([s({type:String})],b.prototype,"offIcon",void 0),e([s({type:String})],b.prototype,"ariaLabelOn",void 0),e([s({type:String})],b.prototype,"ariaLabelOff",void 0),e([s({type:Boolean,reflect:!0})],b.prototype,"on",void 0),e([n("mwc-ripple")],b.prototype,"ripple",void 0),e([t()],b.prototype,"shouldRenderRipple",void 0),e([l({passive:!0})],b.prototype,"handleRippleMouseDown",null),e([l({passive:!0})],b.prototype,"handleRippleTouchStart",null);let R=class extends b{};R.styles=[h],R=e([u("mwc-icon-button-toggle")],R);export{R as IconButtonToggle};
