import logo from "../../images/logo.png";
import styles from "./NavBar.styles";

export const NavBar = () => {
  return (
    <styles.NavBarWrapper>
      <img class="logo" src={logo} alt="Diamond logo" />
      <styles.LinkWrapper>
        <div class="home">Home</div>
        <div class="record">Record</div>
        <div class="contact">Contact</div>
      </styles.LinkWrapper>
    </styles.NavBarWrapper>
  );
};
