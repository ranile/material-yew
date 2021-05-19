import{_ as e,q as t,p as i,a2 as s,h as a,c as l,a4 as r,f as n,b as d,a3 as o,d as h}from"./core.js";const c={fromAttribute:e=>null!==e&&(""===e||e),toAttribute:e=>"boolean"==typeof e?e?"":null:e};class u extends s{constructor(){super(...arguments),this.rows=2,this.cols=20,this.charCounter=!1}render(){const e=this.charCounter&&-1!==this.maxLength,t=e&&"internal"===this.charCounter,i=e&&!t,s=!!this.helper||!!this.validationMessage||i,r={"mdc-text-field--disabled":this.disabled,"mdc-text-field--no-label":!this.label,"mdc-text-field--filled":!this.outlined,"mdc-text-field--outlined":this.outlined,"mdc-text-field--end-aligned":this.endAligned,"mdc-text-field--with-internal-counter":t};return a`
      <label class="mdc-text-field mdc-text-field--textarea ${l(r)}">
        ${this.renderRipple()}
        ${this.outlined?this.renderOutline():this.renderLabel()}
        ${this.renderInput()}
        ${this.renderCharCounter(t)}
        ${this.renderLineRipple()}
      </label>
      ${this.renderHelperText(s,i)}
    `}renderInput(){const e=-1===this.minLength?void 0:this.minLength,t=-1===this.maxLength?void 0:this.maxLength,i=this.autocapitalize?this.autocapitalize:void 0;return a`
      <textarea
          aria-labelledby="label"
          class="mdc-text-field__input"
          .value="${r(this.value)}"
          rows="${this.rows}"
          cols="${this.cols}"
          ?disabled="${this.disabled}"
          placeholder="${this.placeholder}"
          ?required="${this.required}"
          ?readonly="${this.readOnly}"
          minlength="${n(e)}"
          maxlength="${n(t)}"
          name="${n(""===this.name?void 0:this.name)}"
          inputmode="${n(this.inputMode)}"
          autocapitalize="${n(i)}"
          @input="${this.handleInputChange}"
          @blur="${this.onInputBlur}">
      </textarea>`}}e([t("textarea")],u.prototype,"formElement",void 0),e([i({type:Number})],u.prototype,"rows",void 0),e([i({type:Number})],u.prototype,"cols",void 0),e([i({converter:c})],u.prototype,"charCounter",void 0);const p=d`.mdc-text-field{height:100%}.mdc-text-field__input{resize:none}`;let m=class extends u{};m.styles=[o,p],m=e([h("mwc-textarea")],m);export{m as TextArea};
