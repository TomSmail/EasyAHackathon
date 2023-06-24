import logo from "../../images/logo.png";
import styles from "./NavBar.styles";

export const NavBar = () => {
  return (
    <styles.NavBarWrapper>
      <img className="logo" src={logo} alt="Diamond logo" />
      <styles.LinkWrapper>
        <div className="home">Home</div>
        <div className="record">Record</div>
        <div className="contact">Contact</div>
      </styles.LinkWrapper>
    </styles.NavBarWrapper>
  );
};
