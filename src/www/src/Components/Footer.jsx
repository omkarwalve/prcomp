import React from 'react';
import './Footer.css';

export default class Homefooter extends React.Component {

    render() {
        return (
            <div className="footer">
                <div className="footer-grid">
                    <div className="banner-section">
                        <img alt="error" src="_kilowog_.svg" className="website-logo"/>
                        <div className="region-section">
                            <a className="footer-link-normal" href="#">
                                <img alt="error" src="/globe.svg" className="footer-icon" />
                                    IND
                            </a>
                            <select className="language-selector">
                                <option selected value="english">ENGLISH</option>
                                <option value="hindi">हिंदी</option>
                            </select>
                        </div>
                    </div>
                    <div className="service-section">
                        <a className="footer-link-normal" href="#">Customer Service</a>
                        <a className="footer-link-normal" href="#">About Us</a>
                        <a className="footer-link-normal" href="#">Help</a>
                        <a className="footer-link-normal" href="#">Terms and Conditions</a>
                    </div>
                </div>
            </div>
        )
    }
}
