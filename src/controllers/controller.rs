pub(crate) trait Controller {
    async fn handle(&self) -> anyhow::Result<()>;
}
