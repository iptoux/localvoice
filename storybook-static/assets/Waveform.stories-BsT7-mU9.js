import{j as e}from"./jsx-runtime-u17CrQMm.js";import{r as l}from"./iframe-r45v2zDZ.js";import{W as m}from"./Waveform-D3VnCoMS.js";import"./preload-helper-PPVm8Dsz.js";const q={title:"Waveform",component:m,tags:["autodocs"],parameters:{layout:"centered",backgrounds:{default:"dark"}}},o={decorators:[n=>{const{useAppStore:s}=require("../stores/app-store"),{mockInvoke:r}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=r,s.setState({audioLevel:.5}),e.jsx(n,{})}]},a={decorators:[n=>{const{useAppStore:s}=require("../stores/app-store"),{mockInvoke:r}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=r,s.setState({audioLevel:.9}),e.jsx(n,{})}]},c={decorators:[n=>{const{useAppStore:s}=require("../stores/app-store"),{mockInvoke:r}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=r,s.setState({audioLevel:.2}),e.jsx(n,{})}]},i={decorators:[n=>{const{useAppStore:s}=require("../stores/app-store"),{mockInvoke:r}=require("../mocks/tauri");return require("@tauri-apps/api/core").invoke=r,s.setState({audioLevel:0}),e.jsx(n,{})}]},u={parameters:{layout:"centered"},render:()=>{const{useAppStore:n}=require("../stores/app-store"),{mockInvoke:s}=require("../mocks/tauri");require("@tauri-apps/api/core").invoke=s;const r=l.useRef(null),t=l.useRef(0);return l.useEffect(()=>(r.current=setInterval(()=>{t.current+=.1;const d=(Math.sin(t.current)+1)/2;n.setState({audioLevel:d})},100),()=>{r.current&&clearInterval(r.current)}),[]),e.jsxs("div",{className:"space-y-4",children:[e.jsx("h3",{className:"text-sm text-zinc-400",children:"Dynamic Audio Level"}),e.jsx(m,{})]})}},p={parameters:{layout:"fullscreen"},render:()=>{const{useAppStore:n}=require("../stores/app-store"),{mockInvoke:s}=require("../mocks/tauri");require("@tauri-apps/api/core").invoke=s;const r=[0,.2,.4,.6,.8,1];return e.jsxs("div",{className:"p-8 space-y-8 bg-zinc-950 min-h-screen",children:[e.jsx("h2",{className:"text-xl font-semibold text-white",children:"Waveform Levels"}),e.jsx("div",{className:"grid grid-cols-2 md:grid-cols-3 gap-8",children:r.map(t=>(n.setState({audioLevel:t}),e.jsxs("div",{className:"space-y-2",children:[e.jsxs("h3",{className:"text-sm text-zinc-400",children:[Math.round(t*100),"%"]}),e.jsx("div",{className:"bg-zinc-900 p-4 rounded-xl",children:e.jsx(m,{})})]},t)))})]})}};o.parameters={...o.parameters,docs:{...o.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      audioLevel: 0.5
    });
    return <Story />;
  }]
}`,...o.parameters?.docs?.source}}};a.parameters={...a.parameters,docs:{...a.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      audioLevel: 0.9
    });
    return <Story />;
  }]
}`,...a.parameters?.docs?.source}}};c.parameters={...c.parameters,docs:{...c.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      audioLevel: 0.2
    });
    return <Story />;
  }]
}`,...c.parameters?.docs?.source}}};i.parameters={...i.parameters,docs:{...i.parameters?.docs,source:{originalSource:`{
  decorators: [Story => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    useAppStore.setState({
      audioLevel: 0
    });
    return <Story />;
  }]
}`,...i.parameters?.docs?.source}}};u.parameters={...u.parameters,docs:{...u.parameters?.docs,source:{originalSource:`{
  parameters: {
    layout: "centered"
  },
  render: () => {
    const {
      useAppStore
    } = require("../stores/app-store");
    const {
      mockInvoke
    } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);
    const phaseRef = useRef(0);
    useEffect(() => {
      timerRef.current = setInterval(() => {
        phaseRef.current += 0.1;
        const level = (Math.sin(phaseRef.current) + 1) / 2;
        useAppStore.setState({
          audioLevel: level
        });
      }, 100);
      return () => {
        if (timerRef.current) clearInterval(timerRef.current);
      };
    }, []);
    return <div className="space-y-4">\r
        <h3 className="text-sm text-zinc-400">Dynamic Audio Level</h3>\r
        <Waveform />\r
      </div>;
  }
}`,...u.parameters?.docs?.source}}};p.parameters={...p.parameters,docs:{...p.parameters?.docs,source:{originalSource:`{
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
    const levels = [0, 0.2, 0.4, 0.6, 0.8, 1.0];
    return <div className="p-8 space-y-8 bg-zinc-950 min-h-screen">\r
        <h2 className="text-xl font-semibold text-white">Waveform Levels</h2>\r
        <div className="grid grid-cols-2 md:grid-cols-3 gap-8">\r
          {levels.map(level => {
          useAppStore.setState({
            audioLevel: level
          });
          return <div key={level} className="space-y-2">\r
                <h3 className="text-sm text-zinc-400">{Math.round(level * 100)}%</h3>\r
                <div className="bg-zinc-900 p-4 rounded-xl">\r
                  <Waveform />\r
                </div>\r
              </div>;
        })}\r
        </div>\r
      </div>;
  }
}`,...p.parameters?.docs?.source}}};const x=["Default","HighLevel","LowLevel","Silent","DynamicLevel","AllLevels"];export{p as AllLevels,o as Default,u as DynamicLevel,a as HighLevel,c as LowLevel,i as Silent,x as __namedExportsOrder,q as default};
