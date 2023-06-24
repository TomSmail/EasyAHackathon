import styled from "styled-components";

const NavBarWrapper = styled.div`
  display: flex;
  height: 80px;
  justify-content: space-between;
  background-color: #52616b;

  img {
    height: 80px;
  }
`;

const LinkWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
  margin-right: 50px;
  gap: 30px;
`;

const styles = { NavBarWrapper, LinkWrapper };

export default styles;
