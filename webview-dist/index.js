function e(e,r,n,t){return new(n||(n=Promise))((function(o,u){function c(e){try{a(t.next(e))}catch(e){u(e)}}function i(e){try{a(t.throw(e))}catch(e){u(e)}}function a(e){var r;e.done?o(e.value):(r=e.value,r instanceof n?r:new n((function(e){e(r)}))).then(c,i)}a((t=t.apply(e,r||[])).next())}))}function r(e,r){var n,t,o,u={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]},c=Object.create(("function"==typeof Iterator?Iterator:Object).prototype);return c.next=i(0),c.throw=i(1),c.return=i(2),"function"==typeof Symbol&&(c[Symbol.iterator]=function(){return this}),c;function i(i){return function(a){return function(i){if(n)throw new TypeError("Generator is already executing.");for(;c&&(c=0,i[0]&&(u=0)),u;)try{if(n=1,t&&(o=2&i[0]?t.return:i[0]?t.throw||((o=t.return)&&o.call(t),0):t.next)&&!(o=o.call(t,i[1])).done)return o;switch(t=0,o&&(i=[2&i[0],o.value]),i[0]){case 0:case 1:o=i;break;case 4:return u.label++,{value:i[1],done:!1};case 5:u.label++,t=i[1],i=[0];continue;case 7:i=u.ops.pop(),u.trys.pop();continue;default:if(!(o=u.trys,(o=o.length>0&&o[o.length-1])||6!==i[0]&&2!==i[0])){u=0;continue}if(3===i[0]&&(!o||i[1]>o[0]&&i[1]<o[3])){u.label=i[1];break}if(6===i[0]&&u.label<o[1]){u.label=o[1],o=i;break}if(o&&u.label<o[2]){u.label=o[2],u.ops.push(i);break}o[2]&&u.ops.pop(),u.trys.pop();continue}i=r.call(e,u)}catch(e){i=[6,e],t=0}finally{n=o=0}if(5&i[0])throw i[1];return{value:i[0]?i[1]:void 0,done:!0}}([i,a])}}}async function n(e,r={},n){return window.__TAURI_INTERNALS__.invoke(e,r,n)}function t(t){return e(this,void 0,void 0,(function(){var e;return r(this,(function(r){switch(r.label){case 0:return r.trys.push([0,2,,3]),[4,n("plugin:openurl|open_url",{url:t})];case 1:return r.sent(),[3,3];case 2:return e=r.sent(),console.error(e),[3,3];case 3:return[2]}}))}))}"function"==typeof SuppressedError&&SuppressedError,"function"==typeof SuppressedError&&SuppressedError;export{t as open_url};
