import React, {useEffect, useRef, useState} from 'react';
import styled from 'styled-components';
import SyosetuRq from "./syosetu/syosetuApi";

const MBVPDiv = styled.div`
  position: relative;
  height: 100%;
  width: 90%;
  left: 5%;
  overflow: scroll;
  display: flex;
  flex-flow: wrap;
  justify-content: center;
  box-sizing: border-box;
  padding: ${props => props.paddingPx}px 0px;
  

  mask-image: linear-gradient(to bottom, transparent 2%, black 4%, black 96%, transparent 98%);
  
  ::-webkit-scrollbar {
    display: none;
  }
`
const BIBackPanel = styled.div`
  position: relative;
  margin: 1rem;
  width: ${props => props.layout?.w || '10rem'};
  height: ${props => props.layout?.h || '15rem'};
  background: #aaaaaaa0;
  border-radius: 0.5rem;
  box-shadow: inset 0.2rem 0.2rem 4px #76767660, inset -0.2rem -0.2rem 4px #B1B1B140;
`
const BIPanel = styled.div`
  position: absolute;
  top: 2.5%;
  left: 5%;
  height: 95%;
  width: 90%;
  border-radius: 0.2rem;
  background: ${props => props.layout?.bgi? `url("${props.layout.bgi}")` : '#f6f6f6d0'};
  transition: 0.2s;
  overflow: hidden;

  :hover {
    top: -8%;
    left: -2.5%;
    width: 105%;
    height: 110%;
    border-radius: 0.5rem;
    background: #e7ded8;
    box-shadow: 0.5rem 1.2rem 10px #423d3d80;
  }
`
const TitleBar = styled.div`
  position: absolute;
  top: 0.2rem;
  width: 100%;
  text-align: center;
`
const InfoBar = styled.div`
  position: absolute;
  bottom: 0.5rem;
  text-align: center;
`
const StatusPanel = styled.div`
  position: absolute;
  width: 100%;
  bottom: 0;
  background: #ef973a60;
  backdrop-filter: blur(40px);
  font-size: 12px;
  transition: height 1s;
  border-radius: 0.5rem;
`


function BIconPanel({title, writer, keyword, review_cnt, length, story}) {

    const [a, b] = useState('0%')

    return(
        <BIBackPanel>
            <BIPanel onMouseEnter={()=>{b('100%')}} onMouseLeave={()=>{b('0%')}}>
                <TitleBar>
                    {title}
                </TitleBar>
                <InfoBar>
                    author: {writer}
                </InfoBar>
                <StatusPanel style={{height: a}}>
                    tag: {keyword}<br/>
                    review: {review_cnt}<br/>
                    length: {length}<br/>
                    intro: {story}
                </StatusPanel>
            </BIPanel>
        </BIBackPanel>
    )
}



function MBookViewPanel(jsonA) {

    const mainBookPanelRef = useRef()
    const [paddingPx, sPaddingPx] = useState(0)
    const [bookIcons, sBookIcons] = useState({})
    // const [reqSt, sReqSt] = useState(0)


    function syosetu(params={}) {

        const resp = SyosetuRq(params);
        (async function (){
            const bookArray = await resp
            delete bookArray[0]

            sBookIcons(bookArray)
        })();
    }

    // useEffect(()=>{
    //
    // }, [bookIcons])

    useEffect(()=>{
        syosetu(jsonA.jsonA)
    }, [jsonA])



    useEffect(()=>{
        sPaddingPx(mainBookPanelRef.current.offsetHeight * 0.05)
        syosetu()
    }, [])


    // console.log(bookIcons)
    return(
        <MBVPDiv ref={mainBookPanelRef} paddingPx={paddingPx}>
            {Object.entries(bookIcons).map(([k, v]) =>
                <BIconPanel key={k} {...v}/>
            )}

        </MBVPDiv>
    )
}

export default MBookViewPanel