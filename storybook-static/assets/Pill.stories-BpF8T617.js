const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["./event-DdXCvtvw.js","./jsx-runtime-u17CrQMm.js","./iframe-r45v2zDZ.js","./preload-helper-PPVm8Dsz.js","./iframe-DV_WMNXf.css","./Waveform-D3VnCoMS.js","./tauri-BE3wUhPd.js"])))=>i.map(i=>d[i]);
import{j as t}from"./jsx-runtime-u17CrQMm.js";import{r as i}from"./iframe-r45v2zDZ.js";import{c as D,u as l,W as $}from"./Waveform-D3VnCoMS.js";import{_ as W}from"./preload-helper-PPVm8Dsz.js";import{f as m}from"./tauri-BE3wUhPd.js";async function p(e,r={},n){return window.__TAURI_INTERNALS__.invoke(e,r,n)}const z=()=>p("get_settings"),O=(e,r)=>p("update_setting",{key:e,value:r}),F=()=>p("open_main_window"),A=async e=>{const{emit:r}=await W(async()=>{const{emit:n}=await import("./event-DdXCvtvw.js");return{emit:n}},__vite__mapDeps([0,1,2,3,4,5,6]),import.meta.url);await p("open_main_window"),setTimeout(()=>r("navigate-to",e),100)},V=()=>p("expand_pill"),I=()=>p("collapse_pill"),Q=()=>p("start_recording"),U=()=>p("stop_recording"),P=D(e=>({settings:{},loading:!1,load:async()=>{e({loading:!0});try{const r=await z();e({settings:r})}finally{e({loading:!1})}},update:async(r,n)=>{await O(r,n),e(s=>({settings:{...s.settings,[r]:n}}))}}));function L(){const e=l(a=>a.recordingState),r=l(a=>a.lastTranscription),n=l(a=>a.lastOutputResult),s=P(a=>a.load),c=P(a=>a.settings["transcription.default_language"]||"auto");i.useEffect(()=>{s()},[s]);const u=r?.modelId??"—",d=i.useMemo(()=>r?.cleanedText.split(/\s+/).filter(Boolean).length??0,[r?.cleanedText]),g=r?.cleanedText??"",x=e==="listening",o=e==="processing",w=e==="idle",C=()=>{x?U().catch(console.error):w&&Q().catch(console.error)},M=a=>{O("transcription.default_language",a).then(s)},B=()=>{g&&navigator.clipboard.writeText(g).catch(console.error)};return t.jsxs("div",{className:"flex flex-col gap-2 px-3 pt-1 pb-2 text-foreground text-xs select-none overflow-hidden",children:[t.jsx("div",{className:"bg-foreground/10 rounded-md px-2.5 py-2 max-h-20 overflow-y-auto text-[11px] leading-relaxed text-foreground/90 contain-layout-paint",children:g||t.jsx("span",{className:"text-foreground/40 italic",children:"No transcript yet"})}),t.jsxs("div",{className:"flex items-center gap-2",children:[t.jsx(H,{language:c}),t.jsx("span",{className:"text-foreground/40 truncate text-[10px]",children:u}),d>0&&t.jsxs("span",{className:"text-foreground/40 text-[10px] ml-auto",children:[d," ",d===1?"word":"words"]})]}),t.jsx("div",{className:"flex items-center gap-1",children:["auto","de","en"].map(a=>t.jsx("button",{onClick:()=>M(a),className:`px-2 py-0.5 rounded text-[10px] font-semibold uppercase transition-colors ${c===a?"bg-foreground/20 text-foreground":"bg-foreground/5 text-foreground/40 hover:bg-foreground/10 hover:text-foreground/70"}`,children:a},a))}),t.jsx("button",{onClick:C,disabled:o,className:`w-full py-1.5 rounded-md text-[11px] font-semibold transition-all ${x?"bg-red-500 hover:bg-red-400 text-white":o?"bg-foreground/10 text-foreground/30 cursor-not-allowed":"bg-foreground/15 hover:bg-foreground/25 text-foreground"}`,children:x?"Stop Recording":o?"Transcribing…":"Start Recording"}),t.jsxs("div",{className:"flex items-center gap-1.5",children:[t.jsx(_,{label:"Copy",disabled:!g,onClick:B}),t.jsx(_,{label:"History",onClick:()=>A("/history")}),t.jsx(_,{label:"Settings",onClick:()=>A("/settings")})]}),n&&t.jsx("div",{className:`text-center text-[10px] py-0.5 rounded ${n.success?"text-green-600 dark:text-green-300/70":"text-rose-600 dark:text-rose-300/70"}`,children:n.success?n.mode==="insert"?"Inserted into app":"Copied to clipboard":"Output failed"})]})}const H=i.memo(function({language:r}){return t.jsx("span",{className:"bg-foreground/15 text-foreground/80 px-1.5 py-0.5 rounded text-[10px] font-mono uppercase",children:r})},(e,r)=>e.language===r.language),_=i.memo(function({label:r,onClick:n,disabled:s}){return t.jsx("button",{onClick:n,disabled:s,className:"flex-1 py-1 rounded bg-foreground/5 text-foreground/50 hover:bg-foreground/10 hover:text-foreground/80 text-[10px] transition-colors disabled:opacity-30 disabled:cursor-not-allowed",children:r})},(e,r)=>e.label===r.label&&e.disabled===r.disabled);L.__docgenInfo={description:"",methods:[],displayName:"ExpandedPill"};const Y={idle:"bg-card",listening:"bg-red-600",processing:"bg-amber-500",success:"bg-green-600",error:"bg-rose-700"},R=3e3;function N(){const e=l(o=>o.recordingState),r=l(o=>o.setRecordingState),n=l(o=>o.recordingError),s=l(o=>o.isPillExpanded),c=l(o=>o.setIsPillExpanded),[u,d]=i.useState(!1);i.useEffect(()=>{if(e!=="success"){d(!1);return}const o=setTimeout(()=>d(!0),R-300),w=setTimeout(()=>{r("idle"),d(!1)},R);return()=>{clearTimeout(o),clearTimeout(w)}},[e,r]);const g=o=>{o.preventDefault(),s?I().then(()=>c(!1)):V().then(()=>c(!0))},x=()=>{s&&I().then(()=>c(!1)),F()};return i.useEffect(()=>{const o=()=>{s&&I().then(()=>c(!1))};return window.addEventListener("blur",o),()=>window.removeEventListener("blur",o)},[s,c]),t.jsxs("div",{className:`
        w-full rounded-tl-2xl rounded-br-2xl select-none cursor-default overflow-hidden
        ${Y[e]}
        text-foreground shadow-lg border border-foreground/20
        transition-all duration-300 ease-in-out
        ${u?"opacity-80":"opacity-100"}
      `,children:[t.jsx("div",{"data-tauri-drag-region":!0,onContextMenu:g,onDoubleClick:x,className:"flex items-center gap-3 px-4 h-16 text-sm font-medium",children:e==="idle"?t.jsx(G,{}):t.jsxs(t.Fragment,{children:[t.jsx(Z,{state:e}),t.jsx("span",{"data-tauri-drag-region":!0,className:"flex-1 truncate contain-layout-paint",children:e==="error"&&n?n:e==="success"?t.jsx(J,{}):e==="listening"?t.jsx($,{}):"Transcribing…"}),e==="listening"&&t.jsx(ee,{})]})}),s&&t.jsx(L,{})]})}function G(){const e=l(n=>n.lastTranscription),r=i.useMemo(()=>e?.cleanedText?e.cleanedText.trim().split(/\s+/).filter(Boolean).length:void 0,[e?.cleanedText]);return t.jsxs("div",{"data-tauri-drag-region":!0,className:"flex items-center justify-start gap-3 w-full",children:[t.jsx("img",{"data-tauri-drag-region":!0,src:"/localvoice_appiconbadge_transparent.png.png",alt:"LocalVoice",className:"w-8 h-8 flex-shrink-0 object-contain"}),t.jsx("span",{"data-tauri-drag-region":!0,className:"text-base font-bold tracking-tight bg-gradient-to-r from-foreground to-foreground/60 bg-clip-text text-transparent",children:"LocalVoice"}),r!==void 0&&r>0&&t.jsxs("span",{"data-tauri-drag-region":!0,className:"ml-auto text-xs text-foreground/30 tabular-nums",children:[r,"w"]})]})}function J(){const e=l(u=>u.lastTranscription),r=l(u=>u.lastOutputResult),n=e?.cleanedText??"Done",s=i.useMemo(()=>n.length>32?n.slice(0,30)+"…":n,[n]),c=r?.mode==="insert"?"Inserted":"Copied";return t.jsxs("span",{"data-tauri-drag-region":!0,className:"flex items-center gap-3 min-w-0",children:[t.jsx(X,{label:c,success:r?.success??!0}),t.jsx("span",{"data-tauri-drag-region":!0,className:"truncate",title:n,children:s})]})}function K({label:e,success:r}){return t.jsx("span",{"data-tauri-drag-region":!0,className:`
        flex-shrink-0 text-xs px-1.5 py-0.5 rounded font-semibold
        ${r?"bg-white/20 text-white":"bg-rose-900/60 text-rose-200"}
      `,children:r?e:"Failed"})}const X=i.memo(K,(e,r)=>e.label===r.label&&e.success===r.success),Z=i.memo(function({state:r}){const s=r!=="idle"?"white":"currentColor";switch(r){case"listening":return t.jsx("img",{"data-tauri-drag-region":!0,src:"/localvoice_appiconbadge_transparent.png.png",alt:"LocalVoice",className:"w-8 h-8 flex-shrink-0 object-contain"});case"processing":return t.jsx("div",{"data-tauri-drag-region":!0,className:"w-4 h-4 flex-shrink-0 border-2 border-white/80 border-t-transparent rounded-full animate-spin"});case"success":return t.jsx("svg",{"data-tauri-drag-region":!0,className:"w-4 h-4 flex-shrink-0",viewBox:"0 0 16 16",fill:"none",stroke:s,strokeWidth:"2",strokeLinecap:"round",strokeLinejoin:"round",children:t.jsx("polyline",{points:"2,8 6,12 14,4"})});case"error":return t.jsxs("svg",{"data-tauri-drag-region":!0,className:"w-4 h-4 flex-shrink-0",viewBox:"0 0 16 16",fill:"none",stroke:s,strokeWidth:"2",strokeLinecap:"round",children:[t.jsx("line",{x1:"8",y1:"3",x2:"8",y2:"9"}),t.jsx("circle",{cx:"8",cy:"12",r:"1",fill:s})]});default:return t.jsx("img",{"data-tauri-drag-region":!0,src:"/localvoice_appiconbadge_transparent.png.png",alt:"LocalVoice",className:"w-8 h-8 flex-shrink-0"})}});function ee(){const[e,r]=i.useState(0),n=i.useRef(Date.now());i.useEffect(()=>{n.current=Date.now(),r(0);const d=setInterval(()=>{r(Math.floor((Date.now()-n.current)/1e3))},1e3);return()=>clearInterval(d)},[]);const s=Math.floor(e/60),c=e%60,u=`${String(s).padStart(2,"0")}:${String(c).padStart(2,"0")}`;return t.jsx("span",{"data-tauri-drag-region":!0,className:"text-white/90 text-xs tabular-nums flex-shrink-0 font-medium",children:u})}N.__docgenInfo={description:"",methods:[],displayName:"Pill"};const re={title:"Pill",component:N,tags:["autodocs"],parameters:{layout:"centered",backgrounds:{default:"dark"}}},S={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{mockInvoke:n}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=n,r.setState({recordingState:"idle",lastTranscription:null,lastOutputResult:null,recordingError:null,isPillExpanded:!1}),t.jsx(e,{})}]},f={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{mockInvoke:n}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=n,r.setState({recordingState:"idle",lastTranscription:m,lastOutputResult:{mode:"clipboard",success:!0},recordingError:null,isPillExpanded:!1}),t.jsx(e,{})}]},k={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{mockInvoke:n}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=n,r.setState({recordingState:"listening",lastTranscription:null,lastOutputResult:null,recordingError:null,isPillExpanded:!1,audioLevel:.7}),t.jsx(e,{})}]},v={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{mockInvoke:n}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=n,r.setState({recordingState:"processing",lastTranscription:null,lastOutputResult:null,recordingError:null,isPillExpanded:!1}),t.jsx(e,{})}]},h={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{mockInvoke:n}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=n,r.setState({recordingState:"success",lastTranscription:m,lastOutputResult:{mode:"clipboard",success:!0},recordingError:null,isPillExpanded:!1}),t.jsx(e,{})}]},b={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{mockInvoke:n}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=n,r.setState({recordingState:"success",lastTranscription:m,lastOutputResult:{mode:"insert",success:!0},recordingError:null,isPillExpanded:!1}),t.jsx(e,{})}]},E={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{mockInvoke:n}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=n,r.setState({recordingState:"error",lastTranscription:null,lastOutputResult:null,recordingError:"Microphone not available",isPillExpanded:!1}),t.jsx(e,{})}]},q={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{mockInvoke:n}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=n,r.setState({recordingState:"error",lastTranscription:m,lastOutputResult:{mode:"clipboard",success:!1,error:"Clipboard access denied"},recordingError:"Output failed",isPillExpanded:!1}),t.jsx(e,{})}]},j={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{useSettingsStore:n}=require("../stores/settings-store"),{mockInvoke:s}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=s,r.setState({recordingState:"idle",lastTranscription:m,lastOutputResult:{mode:"clipboard",success:!0},recordingError:null,isPillExpanded:!0}),n.setState({settings:{"transcription.default_language":"auto"}}),t.jsx(e,{})}]},y={decorators:[e=>{const{useAppStore:r}=require("../stores/app-store"),{useSettingsStore:n}=require("../stores/settings-store"),{mockInvoke:s}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=s,r.setState({recordingState:"listening",lastTranscription:null,lastOutputResult:null,recordingError:null,isPillExpanded:!0,audioLevel:.5}),n.setState({settings:{"transcription.default_language":"de"}}),t.jsx(e,{})}]},T={parameters:{layout:"fullscreen"},render:()=>{const{useAppStore:e}=require("../stores/app-store"),{mockInvoke:r}=require("../mocks/tauri");require("@tauri-apps/api/core").invoke=r;const n=["idle","listening","processing","success","error"];return t.jsxs("div",{className:"p-8 space-y-8 bg-zinc-950 min-h-screen",children:[t.jsx("h2",{className:"text-xl font-semibold text-white",children:"Pill States"}),t.jsx("div",{className:"grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",children:n.map(s=>(e.setState({recordingState:s,lastTranscription:s==="success"?m:null,lastOutputResult:s==="success"?{mode:"clipboard",success:!0}:null,recordingError:s==="error"?"Microphone unavailable":null,isPillExpanded:!1,audioLevel:s==="listening"?.6:0}),t.jsxs("div",{className:"space-y-2",children:[t.jsx("h3",{className:"text-sm text-zinc-400 uppercase",children:s}),t.jsx("div",{className:"w-80 h-16 rounded-2xl overflow-hidden shadow-2xl",children:t.jsx(N,{})})]},s)))})]})}};S.parameters={...S.parameters,docs:{...S.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "idle",
      lastTranscription: null,
      lastOutputResult: null,
      recordingError: null,
      isPillExpanded: false
    });
    return <Story />;
  }]
}`,...S.parameters?.docs?.source}}};f.parameters={...f.parameters,docs:{...f.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "idle",
      lastTranscription: mockTranscription,
      lastOutputResult: {
        mode: "clipboard",
        success: true
      },
      recordingError: null,
      isPillExpanded: false
    });
    return <Story />;
  }]
}`,...f.parameters?.docs?.source}}};k.parameters={...k.parameters,docs:{...k.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "listening",
      lastTranscription: null,
      lastOutputResult: null,
      recordingError: null,
      isPillExpanded: false,
      audioLevel: 0.7
    });
    return <Story />;
  }]
}`,...k.parameters?.docs?.source}}};v.parameters={...v.parameters,docs:{...v.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "processing",
      lastTranscription: null,
      lastOutputResult: null,
      recordingError: null,
      isPillExpanded: false
    });
    return <Story />;
  }]
}`,...v.parameters?.docs?.source}}};h.parameters={...h.parameters,docs:{...h.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "success",
      lastTranscription: mockTranscription,
      lastOutputResult: {
        mode: "clipboard",
        success: true
      },
      recordingError: null,
      isPillExpanded: false
    });
    return <Story />;
  }]
}`,...h.parameters?.docs?.source}}};b.parameters={...b.parameters,docs:{...b.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "success",
      lastTranscription: mockTranscription,
      lastOutputResult: {
        mode: "insert",
        success: true
      },
      recordingError: null,
      isPillExpanded: false
    });
    return <Story />;
  }]
}`,...b.parameters?.docs?.source}}};E.parameters={...E.parameters,docs:{...E.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "error",
      lastTranscription: null,
      lastOutputResult: null,
      recordingError: "Microphone not available",
      isPillExpanded: false
    });
    return <Story />;
  }]
}`,...E.parameters?.docs?.source}}};q.parameters={...q.parameters,docs:{...q.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "error",
      lastTranscription: mockTranscription,
      lastOutputResult: {
        mode: "clipboard",
        success: false,
        error: "Clipboard access denied"
      },
      recordingError: "Output failed",
      isPillExpanded: false
    });
    return <Story />;
  }]
}`,...q.parameters?.docs?.source}}};j.parameters={...j.parameters,docs:{...j.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      useSettingsStore
    } = require("../stores/settings-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "idle",
      lastTranscription: mockTranscription,
      lastOutputResult: {
        mode: "clipboard",
        success: true
      },
      recordingError: null,
      isPillExpanded: true
    });
    useSettingsStore.setState({
      settings: {
        "transcription.default_language": "auto"
      }
    });
    return <Story />;
  }]
}`,...j.parameters?.docs?.source}}};y.parameters={...y.parameters,docs:{...y.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      useSettingsStore
    } = require("../stores/settings-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      recordingState: "listening",
      lastTranscription: null,
      lastOutputResult: null,
      recordingError: null,
      isPillExpanded: true,
      audioLevel: 0.5
    });
    useSettingsStore.setState({
      settings: {
        "transcription.default_language": "de"
      }
    });
    return <Story />;
  }]
}`,...y.parameters?.docs?.source}}};T.parameters={...T.parameters,docs:{...T.parameters?.docs,source:{originalSource:`{
  parameters: {
    layout: "fullscreen"
  },
  render: () => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    const states: RecordingState[] = ["idle", "listening", "processing", "success", "error"];
    return <div className="p-8 space-y-8 bg-zinc-950 min-h-screen">\r
        <h2 className="text-xl font-semibold text-white">Pill States</h2>\r
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">\r
          {states.map(state => {
          useAppStore.setState({
            recordingState: state,
            lastTranscription: state === "success" ? mockTranscription : null,
            lastOutputResult: state === "success" ? {
              mode: "clipboard",
              success: true
            } : null,
            recordingError: state === "error" ? "Microphone unavailable" : null,
            isPillExpanded: false,
            audioLevel: state === "listening" ? 0.6 : 0
          });
          return <div key={state} className="space-y-2">\r
                <h3 className="text-sm text-zinc-400 uppercase">{state}</h3>\r
                <div className="w-80 h-16 rounded-2xl overflow-hidden shadow-2xl">\r
                  <Pill />\r
                </div>\r
              </div>;
        })}\r
        </div>\r
      </div>;
  }
}`,...T.parameters?.docs?.source}}};const te=["Idle","IdleWithLastTranscription","Listening","Processing","Success","SuccessInsert","Error","ErrorOutputFailed","ExpandedIdle","ExpandedListening","AllStates"],ce=Object.freeze(Object.defineProperty({__proto__:null,AllStates:T,Error:E,ErrorOutputFailed:q,ExpandedIdle:j,ExpandedListening:y,Idle:S,IdleWithLastTranscription:f,Listening:k,Processing:v,Success:h,SuccessInsert:b,__namedExportsOrder:te,default:re},Symbol.toStringTag,{value:"Module"}));export{ce as P,p as i};
