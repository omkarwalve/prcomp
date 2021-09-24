import React from 'react';
import './Footer.css';

export default class Footer extends React.Component {

    render() {
        return (
            <div className="footer">

                <a className="country" href="#">IND</a>
                <select className="lang">
                    <option selected value="english">ENGLISH</option>
                    <option value="hindi">HINDI</option>
                    {/* <option value="spanish">SPANISH</option> */}
                    {/* <option value=""></option>
                    <option value=""></option>
                    <option value=""></option> */}
                </select>
                <a className="custService" href="#">Customer Service</a>
                <a className="aboutUs" href="#">About Us</a>
                <a className="help" href="#">Help</a>
                <a className="termsCond" href="#">Terms and Conditions</a>

            </div>

        )
    }
}
