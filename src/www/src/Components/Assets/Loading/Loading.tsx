//   ___      _______  _______  ______   ___  __    _  _______ 
//  |   |    |       ||   _   ||      | |   ||  |  | ||       |
//  |   |    |   _   ||  |_|  ||  _    ||   ||   |_| ||    ___|
//  |   |    |  | |  ||       || | |   ||   ||       ||   | __ 
//  |   |___ |  |_|  ||       || |_|   ||   ||  _    ||   ||  |
//  |       ||       ||   _   ||       ||   || | |   ||   |_| |
//  |_______||_______||__| |__||______| |___||_|  |__||_______|

import {memo} from "react";

// SVG
import { ReactComponent as Data } from './assets/datasheet.svg';
import { ReactComponent as Product } from './assets/product.svg';
// CSS
import './loading.css';

const Loading = () => {
    return (
        <div className="loading-container">
            <div className="loader">
                <div className="loader-inner">
                    <span> </span>
                    <span> </span>
                    <div><Data /></div>
                    <span> </span>
                    <span> </span>
                </div>
                {  [0,1,2,3].map(_ => {
                    return (<div><Product /></div>);
                    })
                }
            </div>
            <span className="load-text">
                Fetching
                    <ul className="load-dots">
                        {
                            [1,2,3].map(_ => {
                                return( <li>.</li>);
                            })
                        }
                    </ul>
            </span>
        </div>
    )
}

function dotti(i: number) {
    return { '--i' : i };
}

export default memo(Loading)