/* tslint:disable:no-unused-variable */
import {TestBed, inject} from "@angular/core/testing"
import {ThemeService} from "./theme.service"

describe('ThemeService', () => {
	beforeEach(() => {
		TestBed.configureTestingModule({
			providers: [ThemeService]
		});
	});

	it('should ...', inject([ThemeService], (service: ThemeService) => {
		expect(service).toBeTruthy();
	}));
});
