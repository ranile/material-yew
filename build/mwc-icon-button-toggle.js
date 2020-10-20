import{g as t,m as e,n as s,_ as i,q as n,p as o,o as a,a as r,i as l,e as p,B as d,R as c,j as h,h as u,l as A,d as R}from"./core.js";var g={ICON_BUTTON_ON:"mdc-icon-button--on",ROOT:"mdc-icon-button"},b={ARIA_LABEL:"aria-label",ARIA_PRESSED:"aria-pressed",DATA_ARIA_LABEL_OFF:"data-aria-label-off",DATA_ARIA_LABEL_ON:"data-aria-label-on",CHANGE_EVENT:"MDCIconButtonToggle:change"},f=function(s){function i(t){var n=s.call(this,e(e({},i.defaultAdapter),t))||this;return n.hasToggledAriaLabel=!1,n}return t(i,s),Object.defineProperty(i,"cssClasses",{get:function(){return g},enumerable:!0,configurable:!0}),Object.defineProperty(i,"strings",{get:function(){return b},enumerable:!0,configurable:!0}),Object.defineProperty(i,"defaultAdapter",{get:function(){return{addClass:function(){},hasClass:function(){return!1},notifyChange:function(){},removeClass:function(){},getAttr:function(){return null},setAttr:function(){}}},enumerable:!0,configurable:!0}),i.prototype.init=function(){var t=this.adapter.getAttr(b.DATA_ARIA_LABEL_ON),e=this.adapter.getAttr(b.DATA_ARIA_LABEL_OFF);if(t&&e){if(null!==this.adapter.getAttr(b.ARIA_PRESSED))throw new Error("MDCIconButtonToggleFoundation: Button should not set `aria-pressed` if it has a toggled aria label.");this.hasToggledAriaLabel=!0}else this.adapter.setAttr(b.ARIA_PRESSED,String(this.isOn()))},i.prototype.handleClick=function(){this.toggle(),this.adapter.notifyChange({isOn:this.isOn()})},i.prototype.isOn=function(){return this.adapter.hasClass(g.ICON_BUTTON_ON)},i.prototype.toggle=function(t){if(void 0===t&&(t=!this.isOn()),t?this.adapter.addClass(g.ICON_BUTTON_ON):this.adapter.removeClass(g.ICON_BUTTON_ON),this.hasToggledAriaLabel){var e=t?this.adapter.getAttr(b.DATA_ARIA_LABEL_ON):this.adapter.getAttr(b.DATA_ARIA_LABEL_OFF);this.adapter.setAttr(b.ARIA_LABEL,e||"")}else this.adapter.setAttr(b.ARIA_PRESSED,""+t)},i}(s);class _ extends d{constructor(){super(...arguments),this.mdcFoundationClass=f,this.label="",this.disabled=!1,this.onIcon="",this.offIcon="",this.on=!1,this.shouldRenderRipple=!1,this.rippleHandlers=new c(()=>(this.shouldRenderRipple=!0,this.ripple))}createAdapter(){return Object.assign(Object.assign({},h(this.mdcRoot)),{getAttr:t=>this.mdcRoot.getAttribute(t),setAttr:(t,e)=>{this.mdcRoot.setAttribute(t,e)},notifyChange:t=>{this.dispatchEvent(new CustomEvent("MDCIconButtonToggle:change",{detail:t,bubbles:!0}))}})}handleClick(){this.on=!this.on,this.mdcFoundation.handleClick()}focus(){this.rippleHandlers.startFocus(),this.mdcRoot.focus()}blur(){this.rippleHandlers.endFocus(),this.mdcRoot.blur()}renderRipple(){return u`${this.shouldRenderRipple?u`
            <mwc-ripple
                .disabled="${this.disabled}"
                unbounded>
            </mwc-ripple>`:""}`}render(){return u`
      <button
          class="mdc-icon-button"
          @click="${this.handleClick}"
          aria-hidden="true"
          aria-label="${this.label}"
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
        <span class="mdc-icon-button__icon">
          <slot name="offIcon">
            <i class="material-icons">${this.offIcon}</i>
          </slot>
        </span>
        <span class="mdc-icon-button__icon mdc-icon-button__icon--on">
          <slot name="onIcon">
            <i class="material-icons">${this.onIcon}</i>
          </slot>
        </span>
      </button>`}handleRippleMouseDown(t){const e=()=>{window.removeEventListener("mouseup",e),this.handleRippleDeactivate()};window.addEventListener("mouseup",e),this.rippleHandlers.startPress(t)}handleRippleTouchStart(t){this.rippleHandlers.startPress(t)}handleRippleDeactivate(){this.rippleHandlers.endPress()}handleRippleMouseEnter(){this.rippleHandlers.startHover()}handleRippleMouseLeave(){this.rippleHandlers.endHover()}handleRippleFocus(){this.rippleHandlers.startFocus()}handleRippleBlur(){this.rippleHandlers.endFocus()}}i([n(".mdc-icon-button")],_.prototype,"mdcRoot",void 0),i([o({type:String})],_.prototype,"label",void 0),i([o({type:Boolean,reflect:!0})],_.prototype,"disabled",void 0),i([o({type:String})],_.prototype,"onIcon",void 0),i([o({type:String})],_.prototype,"offIcon",void 0),i([o({type:Boolean,reflect:!0}),a((function(t){this.mdcFoundation.toggle(t)}))],_.prototype,"on",void 0),i([r("mwc-ripple")],_.prototype,"ripple",void 0),i([l()],_.prototype,"shouldRenderRipple",void 0),i([p({passive:!0})],_.prototype,"handleRippleMouseDown",null),i([p({passive:!0})],_.prototype,"handleRippleTouchStart",null);let m=class extends _{};m.styles=A,m=i([R("mwc-icon-button-toggle")],m);export{m as IconButtonToggle};
