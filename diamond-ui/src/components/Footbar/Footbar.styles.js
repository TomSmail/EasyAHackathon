import styled from "styled-components";

const FootbarWrapper = styled.div`
  background-color: #52616b;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
`;

const IconsWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 20px;
  svg {
    fill: white;
  }
`;

const styles = { FootbarWrapper, IconsWrapper };

export default styles;
