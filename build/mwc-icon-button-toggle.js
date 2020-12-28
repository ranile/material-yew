import{m as t,_ as e,q as n,p as o,o as s,a as i,i as a,e as r,B as l,R as p,j as d,h as c,l as h,d as u}from"./core.js";var A=function(t,e){return(A=Object.setPrototypeOf||{__proto__:[]}instanceof Array&&function(t,e){t.__proto__=e}||function(t,e){for(var n in e)e.hasOwnProperty(n)&&(t[n]=e[n])})(t,e)};var f=function(){return(f=Object.assign||function(t){for(var e,n=1,o=arguments.length;n<o;n++)for(var s in e=arguments[n])Object.prototype.hasOwnProperty.call(e,s)&&(t[s]=e[s]);return t}).apply(this,arguments)},R={ICON_BUTTON_ON:"mdc-icon-button--on",ROOT:"mdc-icon-button"},b={ARIA_LABEL:"aria-label",ARIA_PRESSED:"aria-pressed",DATA_ARIA_LABEL_OFF:"data-aria-label-off",DATA_ARIA_LABEL_ON:"data-aria-label-on",CHANGE_EVENT:"MDCIconButtonToggle:change"},g=function(t){function e(n){var o=t.call(this,f(f({},e.defaultAdapter),n))||this;return o.hasToggledAriaLabel=!1,o}return function(t,e){function n(){this.constructor=t}A(t,e),t.prototype=null===e?Object.create(e):(n.prototype=e.prototype,new n)}(e,t),Object.defineProperty(e,"cssClasses",{get:function(){return R},enumerable:!0,configurable:!0}),Object.defineProperty(e,"strings",{get:function(){return b},enumerable:!0,configurable:!0}),Object.defineProperty(e,"defaultAdapter",{get:function(){return{addClass:function(){},hasClass:function(){return!1},notifyChange:function(){},removeClass:function(){},getAttr:function(){return null},setAttr:function(){}}},enumerable:!0,configurable:!0}),e.prototype.init=function(){var t=this.adapter.getAttr(b.DATA_ARIA_LABEL_ON),e=this.adapter.getAttr(b.DATA_ARIA_LABEL_OFF);if(t&&e){if(null!==this.adapter.getAttr(b.ARIA_PRESSED))throw new Error("MDCIconButtonToggleFoundation: Button should not set `aria-pressed` if it has a toggled aria label.");this.hasToggledAriaLabel=!0}else this.adapter.setAttr(b.ARIA_PRESSED,String(this.isOn()))},e.prototype.handleClick=function(){this.toggle(),this.adapter.notifyChange({isOn:this.isOn()})},e.prototype.isOn=function(){return this.adapter.hasClass(R.ICON_BUTTON_ON)},e.prototype.toggle=function(t){if(void 0===t&&(t=!this.isOn()),t?this.adapter.addClass(R.ICON_BUTTON_ON):this.adapter.removeClass(R.ICON_BUTTON_ON),this.hasToggledAriaLabel){var e=t?this.adapter.getAttr(b.DATA_ARIA_LABEL_ON):this.adapter.getAttr(b.DATA_ARIA_LABEL_OFF);this.adapter.setAttr(b.ARIA_LABEL,e||"")}else this.adapter.setAttr(b.ARIA_PRESSED,""+t)},e}(t);class _ extends l{constructor(){super(...arguments),this.mdcFoundationClass=g,this.label="",this.disabled=!1,this.onIcon="",this.offIcon="",this.on=!1,this.shouldRenderRipple=!1,this.rippleHandlers=new p(()=>(this.shouldRenderRipple=!0,this.ripple))}createAdapter(){return Object.assign(Object.assign({},d(this.mdcRoot)),{getAttr:t=>this.mdcRoot.getAttribute(t),setAttr:(t,e)=>{this.mdcRoot.setAttribute(t,e)},notifyChange:t=>{this.dispatchEvent(new CustomEvent("MDCIconButtonToggle:change",{detail:t,bubbles:!0}))}})}handleClick(){this.on=!this.on,this.mdcFoundation.handleClick()}focus(){this.rippleHandlers.startFocus(),this.mdcRoot.focus()}blur(){this.rippleHandlers.endFocus(),this.mdcRoot.blur()}renderRipple(){return this.shouldRenderRipple?c`
            <mwc-ripple
                .disabled="${this.disabled}"
                unbounded>
            </mwc-ripple>`:""}render(){return c`
      <button
          class="mdc-icon-button"
          @click="${this.handleClick}"
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
      </button>`}handleRippleMouseDown(t){const e=()=>{window.removeEventListener("mouseup",e),this.handleRippleDeactivate()};window.addEventListener("mouseup",e),this.rippleHandlers.startPress(t)}handleRippleTouchStart(t){this.rippleHandlers.startPress(t)}handleRippleDeactivate(){this.rippleHandlers.endPress()}handleRippleMouseEnter(){this.rippleHandlers.startHover()}handleRippleMouseLeave(){this.rippleHandlers.endHover()}handleRippleFocus(){this.rippleHandlers.startFocus()}handleRippleBlur(){this.rippleHandlers.endFocus()}}e([n(".mdc-icon-button")],_.prototype,"mdcRoot",void 0),e([o({type:String})],_.prototype,"label",void 0),e([o({type:Boolean,reflect:!0})],_.prototype,"disabled",void 0),e([o({type:String})],_.prototype,"onIcon",void 0),e([o({type:String})],_.prototype,"offIcon",void 0),e([o({type:Boolean,reflect:!0}),s((function(t){this.mdcFoundation.toggle(t)}))],_.prototype,"on",void 0),e([i("mwc-ripple")],_.prototype,"ripple",void 0),e([a()],_.prototype,"shouldRenderRipple",void 0),e([r({passive:!0})],_.prototype,"handleRippleMouseDown",null),e([r({passive:!0})],_.prototype,"handleRippleTouchStart",null);let m=class extends _{};m.styles=h,m=e([u("mwc-icon-button-toggle")],m);export{m as IconButtonToggle};
