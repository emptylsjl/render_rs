import React, {useEffect, useRef, useState} from 'react';
import styled from 'styled-components';
import { styled as mui_styled } from '@mui/system';

import init, {draw, img_vec, reset_vec, rload} from '../wp_pkg/next.js';
import {Slider} from "@mui/material";

const MCDiv = styled.div`
  position: fixed;
  top: 0;
  right: 0;
  width: 100%;
  height: 100%;
  //opacity: 0;
`
const MCPanel = styled.div`
  position: absolute;
  top: 5%;
  left: 5%;
  width: 90%;
  height: 90%;
  background: #ffffff20;
  border-radius: 0.5rem;
`
const CDiv = styled.div`
  position: absolute;
  height: ${props => props.layout.h || '60%'};
  width: ${props => props.layout.w || '60%'};
  left: ${props => props.layout.x+'%' || 'auto'};
  top: ${props => props.layout.y+'%' || 'auto'};
  ${props => props.layout.float? `float: ${props.layout.float};`  : ''}
  box-sizing: border-box;
  padding: 0.5rem 0.5rem 0.5rem 0.5rem;
`
const CPanel = styled.div`
  position: relative;
  height: 100%;
  width: 100%;
  border-radius: 0.5rem;
  backdrop-filter: blur(10px);
  background: rgba(224, 213, 213, 0.63);
`
const RCanvas = styled.canvas`
  border: 1px solid orange;
`
const InputBox = styled.input`
  position: absolute;
`
// const EmptyDiv = styled.div``;

const PI = Math.PI;
let rctx;
let imgVec;

function CDivPanel({layout, Comp, content, cdRef, index}) {


    return(
        <CDiv layout={layout}>
            <CPanel>

                {/*<Comp {...content}/>*/}
            </CPanel>
        </CDiv>
    )
}


function MContentPanel() {

    let objs = useRef({});
    let rcanvas = useRef();
    let [ct1, setCt1] = useState(2);
    let [ct2, setCt2] = useState(2);
    let [ct3, setCt3] = useState(2);
    let [ct4, setCt4] = useState(1);
    let [dim, setDim] = useState({h: 0, w: 0});
    let [objLi, setObjLi] = useState({})
    let [dlist, setDlist] = useState([""]);

    let option = {
        trans: [
            {t: 3, v: [0, 0, ct1]},
            {t: 1, v: [-0.5, -0.5, -0.5]},
            {t: 2, v: [0.5, 0.5, 0.5]},
            {t: 2, v: [-1, 1,1]},
            // {t: 0, v: [PI*ct1,0,0]},
            // {t: 0, v: [0,PI*ct2,0]},
            // {t: 0, v: [0, 0, PI*ct3]},
            // {t: 2, v: [ct4, ct4, ct4]},
        ],
        cvs_xy: {x: dim.w, y: dim.h}
    }

    function rc_resize() {
        setDim({h: window.innerHeight*0.9, w: window.innerWidth*0.9})
    }

    function rc_draw() {
        if (rctx !== undefined && dim.w !== 0) {
            let a = performance.now();
            imgVec = img_vec(dim.w, dim.h, imgVec);
            dlist.forEach(i => {
                if (objLi[i]?.ptr) {
                    draw(objLi[i], rctx, imgVec, option);
                    console.log(`${i} -- ${performance.now() - a}ms --`)
                }
            })
        }
    }

    console.log(0x5E00 / 0x200)
    console.log(48* 0x200)
    console.log(parseInt(48* 0x200, 10).toString(16))

    function load_obj(li) {
        console.log("load")
        li.forEach(n => {
            let b = fetch(`/static/obj/${n}/${n}.obj`).then(r => r.text())

            new Promise((resolve, reject) => {
                let img = new Image();
                img.onload = function () {
                    let canv = document.createElement("canvas");
                    let rttx = canv.getContext('2d', {willReadFrequently: true});
                    [canv.height, canv.width] = [this.height, this.width];
                    rttx.drawImage(img, 0, 0);
                    resolve(rttx.getImageData(0, 0, this.width, this.height))
                };
                img.onerror = reject;
                img.src = `/static/obj/${n}/${n}_diffuse.png`;
            }).then(async r => {
                objLi[n] = rload(await b, r.data, [r.width, r.height], objLi[n]);
                setObjLi({...objLi})
            })
        })
    }

    useEffect(()=>{
        rc_draw()
    }, [dim, objLi, ct1, ct2, ct3, ct4])

    useEffect(()=>{
        let li = ["african_head"]
        rctx = rcanvas.current.getContext("2d", {willReadFrequently: true});

        (async () => {

            let response = await fetch('/static/pkg/next_bg.wasm');
            let bytes = await response.arrayBuffer();
            await init(bytes);
            load_obj(li)
            setDlist(li)
            rc_resize();
            window.addEventListener('resize', rc_resize)
        })();

        return _ => {window.removeEventListener('resize', rc_resize)}

    }, [])
    return(
        <MCDiv><MCPanel>
            <Slider aria-label="Volume" step={0.01} min={0.01} max={40} onChange={e => {setCt1(e.target.value)}} />
            <RCanvas ref={rcanvas} width={dim.w} height={dim.h}></RCanvas>
        </MCPanel></MCDiv>
    )
}

export default MContentPanel