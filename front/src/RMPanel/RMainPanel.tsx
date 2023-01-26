import React, {useEffect, useRef, useState} from 'react';
import styled from 'styled-components';
// import Observer from "../other/Observer";
import {RMFPanel, RMIcon} from "./RMContentPanel/RMPanelContent";
import anime from "animejs";


const RMDiv = styled.div`
  position: absolute;
  right: 0;
  width: 100%;
  top: 50%;
  height: 90%;
  transform: translateY(-50%) translateX(${(p: any) => p.transX || '5%'});
  border-top-left-radius: 2rem;
  border-bottom-left-radius: 2rem;
  box-shadow: 0 0 1rem 0.2rem #00000080;
  //transition: width 1.5s;
  background: #ffffff80;
  backdrop-filter: blur(8px);
  transition: transform 1s cubic-bezier(.25,1.2,.36,1.0);
  pointer-events: auto;
`
const RMContentPanel = styled.div`
  position: absolute;
  min-height: 100%;
  width: 100%;
  overflow: scroll;
  ::-webkit-scrollbar {display: none;}
  //scroll-behavior: smooth;
  mask-image: linear-gradient(to bottom, transparent 2%, black 5%, black 95%, transparent 98%);
  
`
const RMIDivFixed = styled.div`
  position: Fixed;
  left: 0;
  width: 60px;
  height: 100%;
  overflow: hidden;
`
const RMIDiv = styled.div`
  position: absolute;
  width: 100%;
  height: ${(props: any) => props?.compStyle?.height || '100%'};
  background: #ffffff20;
`
const RMFDiv = styled.div`
  position: absolute;
  left: 60px;
  flex: 1 1 auto;
  width: calc(100% - 60px);
  min-height: 100%;
  padding: 10% 0;
  background: #ffffff20;
`

function RMainFuncPanel() {
    const mainDivRef: any = useRef()
    const iconDivRef: any = useRef()
    const [iconY, sIconY] = useState({})
    const [iconTop, sIconTop] = useState({})
    const [divTrans, sDivTrans] = useState('80%')
    const [iconScroll, sIconScroll] = useState(true)
    const [divTransPx, sDivTransPx] = useState({expand:0, fold:0})
    const [funcDivPos, sFuncDivPos] = useState({})
    const iconDivWid = 60
    const RMPanels: any = {}

    for (let i = 1; i < 10; i++) {
        const compKey = 'p'+i
        RMPanels[compKey] = {
            compStyle: {iconTop, iconY},
            compState: {},
            setState: {sIconTop, sIconY},
            // eslint-disable-next-line react-hooks/rules-of-hooks
            compRef: {panelRef: useRef()},
            content: undefined,
            child: undefined,
            compKey,
            spare: {},
        }
    }

    function foldIcon(funcDivPx: any = funcDivPos) {
        sIconScroll(false)

        let curHeight = RMPanels['p1'].compRef.panelRef.current.offsetTop;
        const nextH = iconDivRef.current.offsetWidth;
        const curScrollPx = mainDivRef.current.scrollTop

        let iconTrans: any = {}

        Object.entries(RMPanels).forEach(([k, v]) => {
            const scrollPx = 0 - funcDivPx[k] + curScrollPx
            iconTrans[k] = scrollPx + curHeight
            curHeight += nextH
        })

        sIconY(iconTrans)
    }


    function rerenderIcon() {

        sIconScroll(true);
        sDivTrans(divTransPx.expand+'px')

        sIconY({})

        // iconDivRef.current?.animate({
        //     scrollTop: [iconDivRef.current?.scrollTop, mainDivRef.current?.scrollTop]
        // }, 500)
        anime({
            targets: iconDivRef.current,
            scrollTop: mainDivRef.current?.scrollTop,
            duration: 500,
            easing: 'cubicBezier(0.25,0.1,0.25,1)'
        })
    }


    function funcDivOnScroll() {
        if (iconScroll) { iconDivRef.current.scrollTop = mainDivRef.current?.scrollTop }
        let a = [1,2,3]
        a.forEach(k => k + k)
    }

    useEffect(() => {
        let fDivWid = mainDivRef.current?.offsetWidth - iconDivWid

        const fPanelBorderRadius = 0.05

        sDivTrans(fDivWid-fDivWid*fPanelBorderRadius*0.9+'px')
        sDivTransPx({expand: fDivWid*fPanelBorderRadius, fold: fDivWid-fDivWid*fPanelBorderRadius*0.9})

        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    useEffect(()=>{
        let divPos: any = {}
        let iconTopPx: any = {}
        console.log(RMPanels)
        Object.entries(RMPanels).forEach(([k, v]: [string, any]) => {
            iconTopPx[k] = divPos[k] = v.compRef.panelRef.current.offsetTop

        })
        sFuncDivPos(divPos)
        sIconTop(iconTopPx)

        foldIcon(divPos)

        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    // return (
    //     <RMDiv transX={divTrans} onMouseLeave={()=>{foldIcon();sDivTrans(divTransPx.fold+'px')}}>
    //         <RMContentPanel ref={mainDivRef} onScroll={()=>funcDivOnScroll()}>
    //             <RMIDivFixed ref={iconDivRef} onMouseEnter={()=>foldIcon()} onClick={()=>sDivTrans(divTransPx.expand+'px')}>
    //                 <RMIDiv compStyle={{height: mainDivRef.current?.scrollHeight+'px'}}>
    //                     {Object.entries(RMPanels).map(([k, v]) => <RMIcon key={k} {...v}/>)}
    //                 </RMIDiv>
    //             </RMIDivFixed>
    //             <RMFDiv onMouseEnter={() => rerenderIcon()}>
    //                 {Object.entries(RMPanels).map(([k, v]) => <RMFPanel key={k} {...v}/>)}
    //             </RMFDiv>
    //         </RMContentPanel>
    //     </RMDiv>
    // )
}

export default RMainFuncPanel