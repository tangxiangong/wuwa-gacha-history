import FetchForm from "./FetchForm";

interface WelcomePageProps {
  onUserAdded: (playerId: string) => void | Promise<void>;
}

export default function WelcomePage(props: WelcomePageProps) {
  return (
    <div class="welcome-page">
      <div class="welcome-card">
        <h2>欢迎使用鸣潮抽卡记录</h2>
        <p class="welcome-hint">
          粘贴从游戏抽卡记录页面抓取的 JSON 参数（包含 playerId、serverId、
          languageCode、recordId），系统会自动拉取所有卡池的记录。
        </p>
        <FetchForm onSuccess={props.onUserAdded} />
      </div>
    </div>
  );
}
