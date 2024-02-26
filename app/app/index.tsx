import { View, Text } from "react-native";
import { styled } from "nativewind";

const StyledText = styled(Text);
const StyledView = styled(View);
const HomePage = () => {
  return (
    <StyledView>
      <StyledText>Home Page</StyledText>
    </StyledView>
  );
};

export default HomePage;
