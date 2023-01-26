import React, {useEffect, useState} from 'react';
import styled from 'styled-components';

const SSPDiv = styled.div`
  
`
const SSearchPanel = styled.div`
  position: relative;
  top: 50px;
  left: 10%;
  width: 80%;
  height: 3rem;
  box-sizing: border-box;
  border-radius: 2rem;
  border: 2px solid #f6f6f6a0;
  box-shadow: 5px 15px 12px #282c3480, inset 1rem 0 10px #00000040;
  background: rgba(241, 234, 227, 0.55);
  
  //display: flex;
  
  //width: 90%;
  //left: 5%;
  //height: 3.2rem;
  //box-shadow: 8px 20px 12px #282c3480, inset 1rem 0 10px #00000040;
`
const PPClickDiv = styled.div`
  position: absolute;
  width: 23%;
  height: 100%;
  background: transparent;
`
const IsWordButton = styled.div`
  position: absolute;
  height: 100%;
  border-top-left-radius: 1.5rem;
  border-bottom-left-radius: 1.5rem;
  background: #ffffffc0;
  transition: 1s;
  //display: flex;
  //align-items: center;
`
const TextPA = styled.p`
  position: absolute;
  top: 50%;
  transform: translateY(-55%);
  font-size: 0.8rem;
  display: inline;
  margin: auto;
  left: 4%;
  pointer-events: none;
`
const TextPB = styled(TextPA)`
  left: 11%;
`
const SSTypePanel = styled.div`
  position: absolute;
  left: 23%;
  height: 100%;
  width: 15%;
  background: #ffffffc0;
  pointer-events: none;
  
  :after {
    content: '';
    position: absolute;
    left: 0;
    top: 20%;
    height: 60%;
    width: 95%;
    border: 2px #242222a0;
    border-style: hidden solid hidden solid;
    pointer-events: none;
  }
`
const SSearchInputBack = styled.input`
  position: absolute;
  left: 38%;
  height: 100%;
  width: 62%;
  border: transparent;
  border-top-right-radius: 2rem;
  border-bottom-right-radius: 2rem;
  background: #ffffffc0;
`
const SSearchInput = styled.input`
  position: absolute;
  left: 38%;
  height: 100%;
  width: 62%;
  border: 0;
  background: transparent;
  text-align: center;
  :focus {outline:none!important;}
`
const TypePanelText = styled.div`
  position: absolute;
  top: 50%;
  transform: translateY(-55%);
  width: 100%;
  text-align: center;
  pointer-events: auto;
  font-size: 8px;
`
const TypePanelList = styled.div`
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 100%;
  height: 200%;
  display: flex;
  flex-direction: column;
  align-items: center;
  pointer-events: none;
`
const TypePanelListItem = styled.div`
  position: relative;
  width: 100%;
  margin: auto;
  text-align: center;
  font-size: 8px;
  pointer-events: auto;
`


function SyosetuSearchPanel(sJsonA) {

    // const [word, sWord] = useState(undefined)
    // const [notword, sNotword] = useState(undefined)
    // const [title, sTitle] = useState(undefined)
    // const [keyword, sKeyword] = useState(undefined)
    // const [wname, sWname] = useState(undefined)
    // const [order, sOrder] = useState(undefined)
    // const [biggenre, sBiggenre] = useState(undefined)
    // const [length, sLength] =useState(undefined)
    //
    // const searchOpts = {word, notword, title, keyword, wname, order, biggenre, length,}

    const [isWord, sIsWord] = useState(true)
    const [typePanelState, sTypePanelState] = useState('title')

    const [searchInput, sSearchInput] = useState('')


    useEffect(() => {
        const delayDebounceFn = setTimeout(() => {
            const rqParam = {
                word: isWord?searchInput:undefined,
                notword: !isWord?searchInput:undefined,
                title: typePanelState==='title'?1:undefined,
                keyword: typePanelState==='keyword'?1:undefined,
                wname: typePanelState==='author'?1:undefined
            }

            sJsonA.sJsonA(rqParam)
        }, 2000)

        return () => clearTimeout(delayDebounceFn)

        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [searchInput, isWord, typePanelState])

    return(
        <SSPDiv>
            <SSearchPanel>
                <IsWordButton style={{left: isWord?'3%':'10%', width: isWord?'20%':'13%'}}/>
                <PPClickDiv onClick={()=>{sIsWord(!isWord)}}/>
                <TextPA>{isWord?'in':'ex'}</TextPA><TextPB>{'clude'}</TextPB>
                <SSTypePanel>
                    {typePanelState?
                        <TypePanelText onClick={()=>{sTypePanelState(undefined)}}>
                            {typePanelState}
                        </TypePanelText>
                        :
                        <TypePanelList>
                            {['all', 'title', 'keyword', 'author'].map(strX =>
                                <TypePanelListItem key={strX} onClick={()=>{sTypePanelState(strX)}}>
                                    {strX}
                                </TypePanelListItem>
                            )}
                        </TypePanelList>
                    }
                </SSTypePanel>
                <SSearchInputBack/>
                <SSearchInput onChange={(e)=>sSearchInput(e.target.value)}/>
            </SSearchPanel>
        </SSPDiv>
    )
}

export default SyosetuSearchPanel