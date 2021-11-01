import React from 'react';
import './Footer.css';

export default class Homefooter extends React.Component {

    render() {
        return (
            <div className="footer">
                <div className="logoSide">
                    <img src="-kilowog-.svg" className="footLogo"/>
                    <div className="regionOpts">
                        <a className="footText" href="#"><img src="/globe.svg" className="footIcon" /> IND</a>
                        <select className="langSelect">
                            <option selected value="english">ENGLISH</option>
                            <option value="hindi">हिंदी</option>
                        </select>
                    </div>
                </div>
                <div className="serviceSide">
                    <a className="footText" href="#">Customer Service</a>
                    <a className="footText" href="#">About Us</a>
                    <a className="footText" href="#">Help</a>
                    <a className="footText" href="#">Terms and Conditions</a>
                </div>
            </div>
        )
    }
}
