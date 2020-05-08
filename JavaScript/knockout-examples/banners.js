let viewModel = {
  showBanner: ko.observable(false),
  showButton: ko.observable(true),
};

function toggleBanner() {
  viewModel.showBanner(true)
  viewModel.showButton(false)
}

ko.applyBindings(viewModel);
