import React from 'react';
import vegetable from "../../public/images/vegetables_64.png";

const styles = {
    main: "p-4 bg-green-primary flex gap-4",
    logo: "content-center",
    title: "self-center self-justify-center text-white text-4xl font-medium",
}

function TopMenu() {
    return (
        <div className={styles.main}>
            <img className={styles.logo} src={vegetable}></img>
            <span className={styles.title}>
                recip<span className="text-yellow-primary">easy</span>
                {/* recip<sup className="text-orange-primary">ez</sup> */}
                {/* recip<sup>ez</sup> */}
            </span>
        </div>
    );
}

export default TopMenu;
