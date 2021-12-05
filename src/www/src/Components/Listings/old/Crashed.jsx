import React from 'react';
import './Crashed.css';
import { ReactComponent as Oops } from './assets/crashed/oops.svg';

export default function Crashed() {
    return(
        <div className="crashed-section">
            <Oops />
            <span className="crashed-message"> Oops.. Failed to get your data..Sorry?</span>
        </div>
    )
}
